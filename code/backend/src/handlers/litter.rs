use actix_web::{
    Responder, get, post,
    web::{self, Json},
};
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    handlers::HttpError,
    models::{self, litter::Litter},
    services::auth::UserSession,
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
    tags: Vec<String>,
}

impl Into<Litter> for LitterData {
    fn into(self) -> Litter {
        Litter {
            lng: self.lng,
            lat: self.lat,
            file: self.file,
            r#type: self.r#type,
            tags: self.tags,
            _id: ObjectId::new(),
            time_stamp: mongodb::bson::DateTime::now(),
        }
    }
}

#[post("/v1/protected/litter")]
pub async fn create_litter(
    data: web::Json<LitterData>,
    db: web::Data<Database>,
    usersession: UserSession,
) -> Result<impl Responder, HttpError> {
    let mut user = match models::user::User::from_id(&db, usersession.id).await {
        Some(u) => u,
        None => {
            log::info!("Not logged in!");
            return Err(HttpError::InvalidCredentials);
        }
    };
    let litter: Litter = data.0.into();
    let id = litter._id.to_hex();
    user.litter.push(litter);
    user.persist(&db).await;
    Ok(web::Json(json!({"id": id})))
}

#[derive(Debug, Serialize)]
pub struct LitterGetData {
    lat: f64,
    lng: f64,
    file: Vec<u8>,
    r#type: String,
    tags: Vec<String>,
    id: String,
    date: String,
}

impl Into<LitterGetData> for Litter {
    fn into(self) -> LitterGetData {
        LitterGetData {
            lat: self.lat,
            lng: self.lng,
            file: self.file,
            r#type: self.r#type,
            tags: self.tags,
            id: self._id.to_hex(),
            date: self.time_stamp.to_string(),
        }
    }
}
/*
 * Body: Json {
 *    lat: number,
 *    lng: number,
 *    file: []bytes
 *    type: "png" | "jpeg"
 *    tags: []string
 * }
 */
#[get("/v1/protected/litter")]
pub async fn get_litter(
    db: web::Data<Database>,
    usersession: UserSession,
) -> Result<Json<Vec<LitterGetData>>, HttpError> {
    let user = match models::user::User::from_id(&db, usersession.id).await {
        Some(u) => u,
        None => {
            log::info!("Not logged in!");
            return Err(HttpError::InvalidCredentials);
        }
    };
    Ok(web::Json(
        user.litter.into_iter().map(|l| l.into()).collect(),
    ))
}
