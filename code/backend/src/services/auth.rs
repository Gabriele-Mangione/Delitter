use std::{
    collections::HashMap,
    str::FromStr,
    time::{Duration, Instant},
};

use futures::future::{Ready, ready};

use actix_web::{FromRequest, HttpRequest, dev::Payload, web};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use log::{error, info};
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

pub async fn signup(
    db: web::Data<Database>,
    user: &str,
    password: &str,
) -> Result<(ObjectId, Jwt), SignupError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let new_user = User {
        _id: None,
        username: user.to_string(),
        password_hash,
    };

    let result_id = new_user.persist(db).await;

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

    let user: User = users.find_one(doc! { "username": &user }).await.unwrap()?;

    let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
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

#[derive(Debug, Clone, Serialize)]
pub struct Jwt(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claims {}
#[derive(Debug, Clone, Serialize)]
struct Extras {
    id: String,
    username: String,
}

impl Jwt {
    fn new(extras: Extras) -> anyhow::Result<Self> {
        let mut header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512);
        header.kid = Some("blabla".to_owned());

        header.extras = HashMap::with_capacity(1);
        header.extras.insert("userid".to_string(), extras.id);
        header
            .extras
            .insert("username".to_string(), extras.username);

        let token = jsonwebtoken::encode(
            &header,
            &Claims {},
            &EncodingKey::from_secret("secret".as_ref()),
        )?;

        Ok(Self(token))
    }
}

struct UserSession {
    id: ObjectId,
}

impl TryInto<ObjectId> for Jwt {
    type Error = jsonwebtoken::errors::Error;

    fn try_into(self) -> Result<ObjectId, Self::Error> {
        let tokendata = jsonwebtoken::decode(
            self.0,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        )?;

        let claims: Claims = tokendata.claims;
        let id = tokendata
            .header
            .extras
            .get("id")
            .expect("a valid token always contains an id");

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

        if let Some(header_value) = auth_header {
            if let Some(token) = header_value.strip_prefix("Bearer ") {
                let id: Result<ObjectId, _> = Jwt(token.to_string()).try_into();

                match id {
                    Ok(id) => return ready(Ok(UserSession { id })),
                    Err(_) => return ready(Err(HttpError::InvalidToken)),
                }
            }
        }

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
