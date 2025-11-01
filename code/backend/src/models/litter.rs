use actix_web::web;
use mongodb::{
    Collection, Database,
    bson::{Binary, Bson, doc, oid::ObjectId},
};
use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Litter {
    pub _id: ObjectId,
    pub lng: f64,
    pub lat: f64,
    pub file: Option<Binary>,
    pub r#type: String,
    pub tags: Vec<String>,
    pub time_stamp: mongodb::bson::DateTime,
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
                "litter.$.tags": &self.tags,
                "litter.$.time_stamp": self.time_stamp,
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
