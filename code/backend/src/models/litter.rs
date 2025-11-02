use actix_web::web;
use mongodb::{
    Collection, Database,
    bson::{self, Binary, doc, oid::ObjectId},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::user::User;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    pub category: String,
    pub material: String,
    pub weight_g_estimate: Option<f64>,
    pub brand: Option<String>,
    pub confidence: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Litter {
    pub _id: ObjectId,
    pub lng: f64,
    pub lat: f64,
    pub file: Option<Binary>,
    pub r#type: String,
    #[serde(default)]
    pub entries: Vec<Entry>,
    pub time_stamp: mongodb::bson::DateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_counts: Option<HashMap<String, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_total_items: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_processing_time_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_model: Option<String>,
}

impl Litter {
    fn collection(db: &web::Data<Database>) -> Collection<User> {
        db.collection::<User>("users")
    }

    pub async fn persist(&self, db: &web::Data<Database>, user_id: ObjectId) -> Option<ObjectId> {
        let collection = Self::collection(&db);

        let filter = doc! {
            "_id": user_id,
            "litter._id": &self._id,
        };

        let update = doc! {
            "$set": {
                "litter.$.lng": self.lng,
                "litter.$.lat": self.lat,
                "litter.$.file": self.file.clone(),  // Updated to use Binary
                "litter.$.type": &self.r#type,
                "litter.$.entries": bson::to_bson(&self.entries).ok()?,
                "litter.$.time_stamp": self.time_stamp,
                "litter.$.analysis_counts": bson::to_bson(&self.analysis_counts).ok()?,
                "litter.$.analysis_total_items": self.analysis_total_items,
                "litter.$.analysis_notes": &self.analysis_notes,
                "litter.$.analysis_processing_time_ms": self.analysis_processing_time_ms,
                "litter.$.analysis_model": &self.analysis_model,
            }
        };

        // Perform the update
        match collection.update_one(filter, update).await {
            Ok(update_result) => {
                if update_result.matched_count == 1 {
                    Some(self._id.clone())
                } else {
                    None
                }
            }
            Err(e) => {
                eprintln!("Error updating litter: {:?}", e);
                None
            }
        }
    }
}
