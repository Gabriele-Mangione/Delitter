use actix_web::{
    Responder, post,
    web::{self},
};
use mongodb::{Database, bson::doc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{handlers::HttpError, services};
use crate::services::auth::SignupError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct SignupData {
    username: String,
    password: String,
}

#[post("/v1/public/auth/signup")]
pub async fn signup(
    data: web::Json<SignupData>,
    db: web::Data<Database>,
) -> Result<impl Responder, HttpError> {
    let res = services::auth::signup(db, &data.username, &data.password).await;

    match res {
        Ok((id, jwt)) => {
            log::info!("Signin after signup successful: {}", id);

            Ok(web::Json(json!({
                "jwt": jwt,
            })))
        }
        Err(err) => {
            log::info!("Signup failed with {:?}", err);

            match err {
                SignupError::UserAlreadyExists => Err(HttpError::UserAlreadyExists),
                SignupError::NetworkError => Err(HttpError::NetworkError),
                SignupError::UnknownError => Err(HttpError::NetworkError), // or InternalServerError, but since NetworkError is 500
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginData {
    username: String,
    password: String,
}

#[post("/v1/public/auth/signin")]
pub async fn signin(
    data: web::Json<LoginData>,
    db: web::Data<Database>,
) -> Result<impl Responder, HttpError> {
    let res = services::auth::signin(db, &data.username, &data.password).await;

    match res {
        Some((id, jwt)) => {
            log::info!("Login successful: {}", id);

            Ok(web::Json(json!({
                "jwt": jwt,
            })))
        }
        None => {
            log::info!("Login failed");

            Err(HttpError::InvalidCredentials)
        }
    }
}
