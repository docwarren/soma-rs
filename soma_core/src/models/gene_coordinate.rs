use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct GeneCoordinate {
    pub gene: String,
    pub chr: String,
    pub begin: u32,
    pub end: u32,
}