use super::blockchain_core::Block;
use std::collections::HashMap;

pub struct BlockIndexer {
    height_index: HashMap<u64, String>,
    hash_index: HashMap<String, u64>,
    tx_index: HashMap<String, u64>,
}

impl BlockIndexer {
    pub fn new() -> Self {
        BlockIndexer {
            height_index: HashMap::new(),
            hash_index: HashMap::new(),
            tx_index: HashMap::new(),
        }
    }

    pub fn index_block(&mut self, block: &Block) {
        self.height_index.insert(block.index, block.hash.clone());
        self.hash_index.insert(block.hash.clone(), block.index);
        for tx in &block.transactions {
            self.tx_index.insert(tx.clone(), block.index);
        }
    }

    pub fn get_block_hash(&self, height: u64) -> Option<String> {
        self.height_index.get(&height).cloned()
    }

    pub fn get_block_height(&self, hash: &str) -> Option<u64> {
        self.hash_index.get(hash).copied()
    }

    pub fn get_tx_block(&self, tx_id: &str) -> Option<u64> {
        self.tx_index.get(tx_id).copied()
    }
}
