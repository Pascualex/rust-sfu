use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Chunk {
    pub is_key: bool,
    pub timestamp: f64,
    pub bytes: Vec<u8>,
}

impl Chunk {
    pub fn new(is_key: bool, timestamp: f64, bytes: Vec<u8>) -> Self {
        Self {
            is_key,
            timestamp,
            bytes,
        }
    }
}
