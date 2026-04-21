use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use crate::core::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block{
    pub index: u64,
    pub prev_hash: String,
    pub timestamp: u128,
    pub nonce: u64,
    pub hash: String,
    pub transactions: Vec<Transaction>
}

impl Block{
    pub fn new(index: u64, prev_hash: String) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

        let mut block = Block {
            index,
            prev_hash,
            timestamp,
            nonce: 0,
            hash: String::new(),
            transactions: Vec::new()
        };

        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String{
        let tx_data = serde_json::to_string(&self.transactions).unwrap();
        let data = format!("{}{}{}{}{}", self.index, self.prev_hash, self.timestamp, self.nonce, tx_data);

        let mut hasher = Sha256::new();
        hasher.update(data);

        format!("{:x}", hasher.finalize())
    }
}
