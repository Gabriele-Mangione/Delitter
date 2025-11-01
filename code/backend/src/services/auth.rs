use std::{
    collections::HashMap,
    str::FromStr,
    time::{Duration, Instant},
};

use actix_web::web;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use log::{error, info};
use mongodb::{
    Database,
    bson::{Document, doc, oid::ObjectId},
};
use password_hash::{SaltString, rand_core::OsRng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    _id: Option<ObjectId>,
    username: String,
    password_hash: String,
}

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
    let users = db.collection::<User>("users");

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

    let insert_result = users.insert_one(&new_user).await;

    if let Err(e) = &insert_result {
        error!("Error when inserting user: {}", e);
    }

    let insert_result = insert_result.map_err(|_| SignupError::UnknownError)?;

    let id = insert_result
        .inserted_id
        .as_object_id()
        .ok_or(SignupError::WrongCredentials)?;

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
