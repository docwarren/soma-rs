use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cytoband {
    pub chromosome: String,
    pub begin: u32,
    pub end: u32,
    pub name: String,
    pub stain: String
}