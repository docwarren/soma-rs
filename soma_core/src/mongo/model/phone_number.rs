use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhoneNumber {
    pub r#type: String,
    pub number: String
}