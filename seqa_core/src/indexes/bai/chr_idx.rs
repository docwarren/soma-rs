use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::indexes::bin::Bin;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChrIdx {
    pub bins: HashMap<u32, Bin>,
    pub intervals: Vec<u64>
}

impl ChrIdx {
    pub fn new() -> Self {
        ChrIdx {
            bins: HashMap::new(),
            intervals: Vec::new(),
        }
    }
}