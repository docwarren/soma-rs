use crate::mongo::model::phone_number::PhoneNumber;
use crate::mongo::model::id_number::IdNumber;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_numbers: Vec<PhoneNumber>,
    pub specialty: String,
    pub level: String,
    pub id_numbers: Vec<IdNumber>
}