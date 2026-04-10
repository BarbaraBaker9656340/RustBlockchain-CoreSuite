use super::blockchain_core::{Block, Blockchain};

pub struct PoWConsensus {
    difficulty: usize,
}

impl PoWConsensus {
    pub fn new(difficulty: usize) -> Self {
        PoWConsensus { difficulty }
    }

    pub fn mine_block(&self, chain: &Blockchain, transactions: Vec<String>) -> Block {
        let prev_block = chain.chain.last().unwrap();
        let index = prev_block.index + 1;
        let timestamp = chrono::Utc::now().timestamp_millis() as u64;
        let prev_hash = prev_block.hash.clone();
        let mut nonce = 0;

        loop {
            let hash = Blockchain::calculate_hash(index, timestamp, &prev_hash, &transactions, nonce);
            if hash.starts_with(&"0".repeat(self.difficulty)) {
                return Block {
                    index,
                    timestamp,
                    prev_hash,
                    hash,
                    transactions,
                    nonce,
                };
            }
            nonce += 1;
        }
    }

    pub fn validate_block(&self, block: &Block) -> bool {
        let hash = Blockchain::calculate_hash(
            block.index,
            block.timestamp,
            &block.prev_hash,
            &block.transactions,
            block.nonce,
        );
        hash == block.hash && hash.starts_with(&"0".repeat(self.difficulty))
    }
}
