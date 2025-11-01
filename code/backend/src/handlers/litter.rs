use actix_web::{
    Responder, get, post,
    web::{self, Json},
};
use mongodb::{Database, bson::doc};
use serde::{Deserialize, Serialize};

use crate::{handlers::HttpError, models, services::auth::UserSession};

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
    let user = models::user::User::from_id(&db, usersession.id).await;
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
) -> Result<Json<Vec<models::litter::Litter>>, HttpError> {
    Ok(web::Json(vec![models::litter::Litter::default()]))
}
