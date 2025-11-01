use actix_web::web;
use log::error;
use mongodb::{Collection, Database, bson::oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub username: String,
    pub password_hash: String,
}

impl User {
    fn collection(db: web::Data<Database>) -> Collection<Self> {
        db.collection::<User>("users")
    }
    pub async fn persist(&self, db: web::Data<Database>) -> Option<ObjectId> {
        match Self::collection(db).insert_one(self).await {
            Ok(res) => res.inserted_id.as_object_id(),
            Err(e) => {
                error!("failed to insert user {:?}", e);
                None
            }
        }
    }
}
