use crate::mongo::model::phone_number::PhoneNumber;
use crate::mongo::model::address::Address;
use crate::mongo::model::insurance_number::InsuranceNumber;
use crate::mongo::model::id_number::IdNumber;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Patient {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub first_name: String,
    pub last_name: String,
    pub dob: String,
    pub sex: String,
    pub address: Address,
    pub phone_numbers: Vec<PhoneNumber>,
    pub insurance_numbers: Vec<InsuranceNumber>,
    pub id_numbers: Vec<IdNumber>,
}