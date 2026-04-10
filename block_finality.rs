use super::blockchain_core::Block;
use std::collections::HashSet;

pub struct BlockFinality {
    confirmed_blocks: HashSet<u64>,
    confirmation_depth: u64,
}

impl BlockFinality {
    pub fn new(depth: u64) -> Self {
        BlockFinality {
            confirmed_blocks: HashSet::new(),
            confirmation_depth: depth,
        }
    }

    pub fn check_finality(&mut self, chain_height: u64, block_height: u64) -> bool {
        if chain_height - block_height >= self.confirmation_depth {
            self.confirmed_blocks.insert(block_height);
            true
        } else {
            false
        }
    }

    pub fn is_finalized(&self, block_height: u64) -> bool {
        self.confirmed_blocks.contains(&block_height)
    }
}
