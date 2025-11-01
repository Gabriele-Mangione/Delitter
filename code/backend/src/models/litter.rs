use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Litter {
    pub _id: ObjectId,
    pub lng: f64,
    pub lat: f64,
    pub file: Vec<u8>,
    pub r#type: String,
    pub tags: Vec<String>,
    pub time_stamp: mongodb::bson::DateTime,
}
