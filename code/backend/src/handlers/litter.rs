use actix_web::{
    Responder, get, post,
    web::{self, Json},
};
use mongodb::{
    Database,
    bson::{Binary, doc, oid::ObjectId},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

use crate::{
    handlers::HttpError,
    models::{self, litter::Litter},
    services::auth::UserSession,
};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LitterData {
    lat: f64,
    lng: f64,
    #[schema(format = "binary")]
    file: Vec<u8>,
    r#type: String,
}

impl Into<Litter> for LitterData {
    fn into(self) -> Litter {
        let file_binary = Binary {
            subtype: mongodb::bson::spec::BinarySubtype::Generic, // Set the correct subtype for your use case
            bytes: self.file.clone(),
        };

        Litter {
            lng: self.lng,
            lat: self.lat,
            file: Some(file_binary),
            r#type: self.r#type,
            entries: vec![],
            _id: ObjectId::new(),
            time_stamp: mongodb::bson::DateTime::now(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LitterCreateResponse {
    id: String,
}

#[utoipa::path(
    post,
    path = "/v1/protected/litter",
    request_body = LitterData,
    responses(
        (status = 200, description = "Litter successfully created", body = LitterCreateResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 500, description = "Network error")
    ),
    tag = "Litter",
    security(
        ("bearer_auth" = [])
    )
)]
#[post("/v1/protected/litter")]
pub async fn create_litter(
    data: web::Json<LitterData>,
    db: web::Data<Database>,
    usersession: UserSession,
) -> Result<impl Responder, HttpError> {
    let file = data.file.clone();

    let mut user = match models::user::User::from_id(&db, usersession.id).await {
        Some(u) => u,
        None => {
            log::info!("Not logged in!");
            return Err(HttpError::InvalidCredentials);
        }
    };

    let mut litter: Litter = data.0.into();
    let id = litter._id.to_hex();

    // Clone what you need because it's moved into the new task
    // let db_clone = db.clone();
    user.litter.push(litter.clone());

    // Spawn a new asynchronous task for analysis
    if let Err(e) = user.persist(&db).await {
        log::error!("Failed to persist user: {:?}", e);
        return Err(HttpError::NetworkError);
    }

    tokio::spawn(async move {
        // Call your analyze function
        let res = match crate::services::analyzer::analyze(file).await {
            Ok(r) => r,
            Err(e) => {
                log::error!("Error while analysing image: {}", e);
                return;
            }
        };

        // Can Metal Pepsi 5g
        // Bottle Plastic Rivella 50g
        //
        // Category  Bottle | Can
        // Material  Plastic | Metal
        // Weigth    50g | 5g
        // brand     Rivella |

        for obj in res {
            litter.entries.push(models::litter::Entry {
                category: obj.category,
                material: obj.material,
                weight: Some(obj.weight_g_estimate),
                brand: obj.brand,
            });
        }

        let _ = litter.persist(&db, usersession.id).await;
    });

    // Immediately return the ID to the client
    Ok(web::Json(json!({ "id": id })))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LitterEntryGetData {
    category: Option<String>,
    material: Option<String>,
    weight: Option<f64>,
    brand: Option<String>,
}
#[derive(Debug, Serialize, ToSchema)]
pub struct LitterGetData {
    lat: f64,
    lng: f64,
    #[schema(format = "binary")]
    file: Vec<u8>,
    r#type: String,
    entries: Vec<LitterEntryGetData>,
    id: String,
    date: String,
}

impl Into<LitterGetData> for Litter {
    fn into(self) -> LitterGetData {
        LitterGetData {
            lat: self.lat,
            lng: self.lng,
            file: self.file.map(|f| f.bytes).unwrap_or_default(),
            r#type: self.r#type,

            entries: self
                .entries
                .into_iter()
                .map(|el| LitterEntryGetData {
                    category: el.category,
                    material: el.material,
                    weight: el.weight,
                    brand: el.brand,
                })
                .collect(),
            id: self._id.to_hex(),
            date: self.time_stamp.to_string(),
        }
    }
}

#[utoipa::path(
    get,
    path = "/v1/protected/litter",
    responses(
        (status = 200, description = "List of litter items", body = Vec<LitterGetData>),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "Litter",
    security(
        ("bearer_auth" = [])
    )
)]
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
