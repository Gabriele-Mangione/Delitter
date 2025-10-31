use actix_web::web;
use argon2::{Argon2, PasswordHasher};
use log::error;
use mongodb::{Database, bson::oid::ObjectId};
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
) -> Result<ObjectId, SignupError> {
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

    let insert_result = users.insert_one(new_user).await;

    if let Err(e) = &insert_result {
        error!("Error when inserting user: {}", e);
    }

    let insert_result = insert_result.map_err(|_| SignupError::UnknownError)?;

    insert_result
        .inserted_id
        .as_object_id()
        .ok_or(SignupError::WrongCredentials)
}
