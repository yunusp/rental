use mongodb::bson::oid::ObjectId;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Car {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub owner_id: ObjectId,
    pub borrower_id: Option<ObjectId>,

    pub name: String,
    pub number: String,
    pub brand: String,
    pub price: u64,
    pub dt: u128,
    pub iat: String, // issued at
    pub ito: String, // issued to (time)
    pub picture: String,
    pub desc: String,
}
