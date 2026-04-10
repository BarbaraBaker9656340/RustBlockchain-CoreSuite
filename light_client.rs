use super::blockchain_core::Block;
use super::merkle_tree::MerkleTree;

pub struct LightClient {
    chain_id: String,
    block_headers: Vec<BlockHeader>,
}

#[derive(Debug, Clone)]
pub struct BlockHeader {
    pub height: u64,
    pub hash: String,
    pub prev_hash: String,
    pub merkle_root: String,
}

impl LightClient {
    pub fn new(chain_id: &str) -> Self {
        LightClient {
            chain_id: chain_id.to_string(),
            block_headers: Vec::new(),
        }
    }

    pub fn add_header(&mut self, header: BlockHeader) {
        self.block_headers.push(header);
    }

    pub fn verify_tx_proof(&self, height: u64, tx_hash: &str, proof: &[String]) -> bool {
        let header = self.block_headers.iter().find(|h| h.height == height)?;
        let mut current = tx_hash.to_string();
        
        for p in proof {
            let combined = format!("{}{}", current, p);
            current = hex::encode(sha2::Sha256::digest(combined.as_bytes()));
        }
        
        Some(&current) == Some(&header.merkle_root)
    }
}
