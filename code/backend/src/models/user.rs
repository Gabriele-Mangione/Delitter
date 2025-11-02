use actix_web::web;
use log::error;
use mongodb::{
    Collection, Database,
    bson::{doc, oid::ObjectId, to_document},
};
use serde::{Deserialize, Serialize};

use super::litter::Litter;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub username: String,
    pub password_hash: String,

    #[serde(default)]
    pub litter: Vec<Litter>,
}

impl User {
    fn collection(db: &web::Data<Database>) -> Collection<Self> {
        db.collection::<User>("users")
    }
    pub async fn persist(&self, db: &web::Data<Database>) -> Result<ObjectId, mongodb::error::Error> {
        let collection = Self::collection(db);
        match self._id {
            Some(id) => match collection
                .update_one(doc! {"_id": id}, doc! {"$set": to_document(self).map_err(|e| mongodb::error::Error::custom(format!("Serialization failed: {}", e)))?})
                .await
            {
                Ok(_) => Ok(id),
                Err(e) => {
                    error!("failed to update user {:?}", e);
                    Err(e)
                }
            },
            None => match collection.insert_one(self).await {
                Ok(res) => res.inserted_id.as_object_id().ok_or_else(|| mongodb::error::Error::custom("Inserted ID is not ObjectId")),
                Err(e) => {
                    error!("failed to insert user {:?}", e);
                    Err(e)
                }
            },
        }
    }

    pub async fn from_id(db: &web::Data<Database>, id: ObjectId) -> Option<Self> {
        match Self::collection(db).find_one(doc! {"_id": id}).await {
            Ok(res) => res,
            Err(e) => {
                error!("failed to find user {:?}", e);
                None
            }
        }
    }
}
