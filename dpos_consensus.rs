use std::collections::HashMap;
use super::blockchain_core::Block;

pub struct DPoSConsensus {
    validators: Vec<String>,
    votes: HashMap<String, u64>,
    max_validators: usize,
}

impl DPoSConsensus {
    pub fn new(max_validators: usize) -> Self {
        DPoSConsensus {
            validators: Vec::new(),
            votes: HashMap::new(),
            max_validators,
        }
    }

    pub fn vote(&mut self, voter: String, validator: String, power: u64) {
        *self.votes.entry(validator).or_insert(0) += power;
        self.update_validators();
    }

    fn update_validators(&mut self) {
        let mut list: Vec<(String, u64)> = self.votes.clone().into_iter().collect();
        list.sort_by(|a, b| b.1.cmp(&a.1));
        self.validators = list.into_iter()
            .take(self.max_validators)
            .map(|(addr, _)| addr)
            .collect();
    }

    pub fn get_next_validator(&self) -> Option<String> {
        self.validators.first().cloned()
    }

    pub fn is_validator(&self, addr: &str) -> bool {
        self.validators.contains(&addr.to_string())
    }
}
