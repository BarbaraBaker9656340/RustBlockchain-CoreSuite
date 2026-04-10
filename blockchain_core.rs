use chrono::prelude::*;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub prev_hash: String,
    pub hash: String,
    pub transactions: Vec<String>,
    pub nonce: u64,
}

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<String>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut chain = Vec::new();
        let genesis = Self::create_genesis_block();
        chain.push(genesis);
        Blockchain {
            chain,
            pending_transactions: Vec::new(),
        }
    }

    fn create_genesis_block() -> Block {
        let timestamp = Utc::now().timestamp_millis() as u64;
        let prev_hash = "0".to_string();
        let transactions = vec!["genesis_block".to_string()];
        let nonce = 0;
        let hash = Self::calculate_hash(0, timestamp, &prev_hash, &transactions, nonce);
        
        Block {
            index: 0,
            timestamp,
            prev_hash,
            hash,
            transactions,
            nonce,
        }
    }

    pub fn calculate_hash(index: u64, timestamp: u64, prev_hash: &str, transactions: &[String], nonce: u64) -> String {
        let input = format!("{}{}{}{:?}{}", index, timestamp, prev_hash, transactions, nonce);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }
}
