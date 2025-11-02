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
            analysis_counts: None,
            analysis_total_items: None,
            analysis_notes: None,
            analysis_processing_time_ms: None,
            analysis_model: None,
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
        // Call image recognition function
        let analysis = match crate::services::image_recognition::analyze_image(file).await {
            Ok(a) => a,
            Err(e) => {
                log::error!("Error while analysing image: {}", e);
                return;
            }
        };

        // Update litter with analysis results
        litter.entries = analysis.entries;
        litter.analysis_counts = analysis.counts;
        litter.analysis_total_items = analysis.total_items;
        litter.analysis_notes = analysis.notes;
        litter.analysis_processing_time_ms = Some(analysis.processing_time_ms);
        litter.analysis_model = Some(analysis.model);

        let _ = litter.persist(&db, usersession.id).await;
    });

    // Immediately return the ID to the client
    Ok(web::Json(json!({ "id": id })))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LitterEntryGetData {
    category: String,
    material: String,
    weight_g_estimate: Option<f64>,
    brand: Option<String>,
    confidence: f64,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    analysis_counts: Option<std::collections::HashMap<String, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    analysis_total_items: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    analysis_notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    analysis_processing_time_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    analysis_model: Option<String>,
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
                    weight_g_estimate: el.weight_g_estimate,
                    brand: el.brand,
                    confidence: el.confidence,
                })
                .collect(),
            id: self._id.to_hex(),
            date: self.time_stamp.to_string(),
            analysis_counts: self.analysis_counts,
            analysis_total_items: self.analysis_total_items,
            analysis_notes: self.analysis_notes,
            analysis_processing_time_ms: self.analysis_processing_time_ms,
            analysis_model: self.analysis_model,
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
