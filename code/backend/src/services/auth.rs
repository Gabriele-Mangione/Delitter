use std::{
    collections::HashMap,
    str::FromStr,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use futures::future::{Ready, ready};

use actix_web::{FromRequest, HttpRequest, dev::Payload, web};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use log::{debug, error, info};
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId},
};
use password_hash::{SaltString, rand_core::OsRng};
use serde::{Deserialize, Serialize};

use crate::{handlers::HttpError, models::user::User};

#[derive(Debug, Clone, Serialize)]
pub enum SignupError {
    WrongCredentials,
    UnknownError,
}

#[derive(Debug, Clone, Serialize)]
pub struct Jwt(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claims {
    exp: u64,
}

#[derive(Debug, Clone, Serialize)]
struct Extras {
    id: String,
    username: String,
}

pub struct UserSession {
    pub id: ObjectId,
}

pub async fn signup(
    db: web::Data<Database>,
    user: &str,
    password: &str,
) -> Result<(ObjectId, Jwt), SignupError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(o) => o.to_string(),
        Err(e) => {
            error!("Password hashing failed: {}", e);
            return Err(SignupError::UnknownError);
        }
    };

    let new_user = User {
        username: user.to_string(),
        password_hash,
        ..Default::default()
    };

    let result_id = new_user.persist(&db).await;

    let id = result_id.ok_or(SignupError::UnknownError)?;

    let jwt = Jwt::new(Extras {
        id: id.to_hex(),
        username: new_user.username,
    });

    if let Err(e) = &jwt {
        error!("Error when inserting user / creating jwt: {}", e);
    }

    let jwt = jwt.map_err(|_| SignupError::UnknownError)?;

    Ok((id, jwt))
}

pub async fn signin(
    db: web::Data<Database>,
    user: &str,
    password: &str,
) -> Option<(ObjectId, Jwt)> {
    let users = db.collection::<User>("users");

    let user: Result<Option<User>, _> = users.find_one(doc! { "username": &user }).await;

    let user = match user {
        Ok(u) => u,
        Err(e) => {
            error!("Error when searching for username in db: {}", e);
            return None;
        }
    };
    let user = match user {
        Some(u) => u,
        None => {
            info!("Username not found in db!");
            return None;
        }
    };

    let parsed_hash = match  PasswordHash::new(&user.password_hash) {
        Ok(o) => o,
        Err(e) => {
            error!("Password hashing failed: {}", e);
            return None;
        }
    };

    let argon2 = Argon2::default();
    if argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_err()
    {
        info!("Invalid credentials!");
        return None;
    }

    let id = user._id.expect("Id is always there when reading");
    let jwt = Jwt::new(Extras {
        id: id.to_hex(),
        username: user.username,
    });

    if let Err(e) = &jwt {
        error!("Error when inserting user / creating jwt: {}", e);
    }

    jwt.ok().map(|jwt| (id, jwt))
}

impl Jwt {
    fn new(extras: Extras) -> anyhow::Result<Self> {
        let mut header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512);
        header.kid = Some("blabla".to_owned());

        header.extras = HashMap::with_capacity(1);
        header.extras.insert("id".to_string(), extras.id);
        header
            .extras
            .insert("username".to_string(), extras.username);

        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        // 1 week in seconds
        let one_week = 7 * 24 * 60 * 60;

        let token = jsonwebtoken::encode(
            &header,
            &Claims {
                exp: now + one_week,
            },
            &EncodingKey::from_secret("secret".as_ref()),
        )?;

        Ok(Self(token))
    }
}

impl TryInto<ObjectId> for Jwt {
    type Error = jsonwebtoken::errors::Error;

    fn try_into(self) -> Result<ObjectId, Self::Error> {
        let tokendata = jsonwebtoken::decode(
            self.0,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::new(jsonwebtoken::Algorithm::HS512),
        )?;
        debug!("Tokendata could be extracted {:?}", tokendata);

        let claims: Claims = tokendata.claims;
        let id = tokendata
            .header
            .extras
            .get("id")
            .ok_or(jsonwebtoken::errors::ErrorKind::InvalidToken)?;

        Ok(ObjectId::from_str(id).expect("a valid jwt cannot contain an invalid mongo id"))
    }
}

impl FromRequest for UserSession {
    type Error = HttpError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok());
        debug!("Entering session check with {:?}", auth_header);

        if let Some(header_value) = auth_header {
            if let Some(token) = header_value.strip_prefix("Bearer ") {
                let id: Result<ObjectId, _> = Jwt(token.to_string()).try_into();

                debug!("Result from JWT {:?}", id);

                match id {
                    Ok(id) => return ready(Ok(UserSession { id })),
                    Err(_) => return ready(Err(HttpError::InvalidToken)),
                }
            }
            debug!("Auth header does not start with 'Bearer '. Request invalid!");
        }
        debug!("No auth header. Request invalid!");

        ready(Err(HttpError::InvalidToken))
    }
}

struct Entry {
    img: Vec<u8>,
    // Count / Kind / Brand
    tags: Vec<(u8, String, String)>,
}

struct Report {
    entries: Vec<Entry>,
    route: Vec<(u8, u8)>,
    distance: u32,
    date: Instant,
    time: Duration,
}
