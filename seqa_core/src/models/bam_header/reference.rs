use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BamReference {
    pub name: String,
    pub length: u32,
}

