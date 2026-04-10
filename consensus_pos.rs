use std::collections::HashMap;
use super::blockchain_core::Block;

pub struct PoSConsensus {
    stakes: HashMap<String, u64>,
    total_stake: u64,
}

impl PoSConsensus {
    pub fn new() -> Self {
        PoSConsensus {
            stakes: HashMap::new(),
            total_stake: 0,
        }
    }

    pub fn stake(&mut self, validator: String, amount: u64) {
        *self.stakes.entry(validator).or_insert(0) += amount;
        self.total_stake += amount;
    }

    pub fn select_validator(&self) -> Option<String> {
        if self.total_stake == 0 {
            return None;
        }
        let mut rand_val = rand::random::<u64>() % self.total_stake;
        for (addr, stake) in &self.stakes {
            if rand_val < *stake {
                return Some(addr.clone());
            }
            rand_val -= stake;
        }
        None
    }

    pub fn validate_block(&self, block: &Block, validator: &str) -> bool {
        self.stakes.contains_key(validator) && *self.stakes.get(validator).unwrap() > 0
    }
}
