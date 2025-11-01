use actix_web::{
    Responder, get, post,
    web::{self},
};
use mongodb::{Database, bson::doc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{handlers::HttpError, services};

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

/*
 * Auth; Bearer askjfriqwur
 * Body: Json {
 *    lat: number,
 *    lng: number,
 *    file: []bytes
 *    type: "png" | "jpeg"
 * }
 */
#[post("/v1/protected/litter")]
pub async fn create_litter(
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
            log::info!("Login failed with {:?}", err);

            Err(HttpError::InvalidCredentials)
        }
    }
}

/*
 * Body: Json {
 *    lat: number,
 *    lng: number,
 *    file: []bytes
 *    type: "png" | "jpeg"
 *
 *    tags: []string
 * }
 */
#[get("/v1/protected/litter")]
pub async fn get_litter() -> Result<impl Responder, HttpError> {
    Ok(web::Json("todo"))
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
