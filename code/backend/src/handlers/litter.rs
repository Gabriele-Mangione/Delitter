use actix_web::{
    Responder, get, post,
    web::{self},
};
use mongodb::{Database, bson::doc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    handlers::HttpError,
    services::{self, auth::UserSession},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct LitterData {
    lat: f64,
    lng: f64,
    file: Vec<u8>,
    r#type: String,
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
    data: web::Json<LitterData>,
    db: web::Data<Database>,
    usersession: UserSession,
) -> Result<impl Responder, HttpError> {
    Ok(web::Json("todo"))
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
pub async fn get_litter(
    db: web::Data<Database>,
    usersession: UserSession,
) -> Result<impl Responder, HttpError> {
    Ok(web::Json("todo"))
}
