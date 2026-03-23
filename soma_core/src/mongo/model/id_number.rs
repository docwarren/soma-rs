use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IdNumber {
    pub r#type: String,
    pub number: String
}