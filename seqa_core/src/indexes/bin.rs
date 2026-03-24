use serde::{Deserialize, Serialize};

use super::chunk::Chunk;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bin {
    pub bin: u32,
    pub chunks: Vec<Chunk>
}