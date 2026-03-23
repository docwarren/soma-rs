use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsuranceNumber {
    pub provider: String,
    pub policy_number: String
}