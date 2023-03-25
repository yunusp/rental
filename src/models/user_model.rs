// use bson::Timestamp;
use mongodb::bson::oid::ObjectId;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub uname: String,
    pub email: String,
    pub phone_number: String,
    pub adhaar_number: String,
    pub pass: String,
    pub photo_id: String,
    pub birthday: String,
}
