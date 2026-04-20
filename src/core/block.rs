use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH, Duration};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block{
    pub index: u64,
    pub prev_hash: String,
    pub timestamp: u128,
    pub nonce: u64,
    pub hash: String
}

impl Block{
    pub fn new(index: u64, prev_hash: String) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

        let mut block = Block {
            index,
            prev_hash,
            timestamp,
            nonce: 0,
            hash: String::new()
        };

        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String{
        let data = format!("{}{}{}{}", self.index, self.prev_hash, self.timestamp, self.nonce);

        let mut hasher = Sha256::new();
        hasher.update(data);

        format!("{:x}", hasher.finalize())
    }
}
