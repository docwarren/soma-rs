use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String
}