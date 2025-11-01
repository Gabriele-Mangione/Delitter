use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Litter {
    pub _id: Option<ObjectId>,
    pub lng: f64,
    pub lat: f64,
}
