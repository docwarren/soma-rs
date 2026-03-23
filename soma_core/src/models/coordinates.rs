use serde::{ Serialize, Deserialize };

// Canonical Coordinates are zero base half open

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CoordinateSystem {
    ZeroBasedHalfOpen,
    OneBasedClosed
}

impl CoordinateSystem {
    pub fn to_canonical(&self, begin: u32, end: u32) -> (u32, u32) {
        match self {
            CoordinateSystem::ZeroBasedHalfOpen => (begin, end),
            CoordinateSystem::OneBasedClosed => (begin - 1, end),
        }
    }

    pub fn from_canonical(&self, begin: u32, end: u32) -> (u32, u32) {
        match self {
            CoordinateSystem::ZeroBasedHalfOpen => (begin, end),
            CoordinateSystem::OneBasedClosed => (begin + 1, end),
        }
    }
}
