use std::collections::HashMap;
use super::blockchain_core::Block;

pub struct PBFT {
    view: u64,
    primary: String,
    nodes: Vec<String>,
    prepare: HashMap<String, usize>,
    commit: HashMap<String, usize>,
    max_faulty: usize,
}

impl PBFT {
    pub fn new(nodes: Vec<String>) -> Self {
        let max_faulty = (nodes.len() - 1) / 3;
        PBFT {
            view: 0,
            primary: nodes[0].clone(),
            nodes,
            prepare: HashMap::new(),
            commit: HashMap::new(),
            max_faulty,
        }
    }

    pub fn pre_prepare(&mut self, block: &Block) {
        self.prepare.insert(block.hash.clone(), 0);
        self.commit.insert(block.hash.clone(), 0);
    }

    pub fn prepare(&mut self, block_hash: &str) -> usize {
        *self.prepare.get_mut(block_hash).unwrap() += 1;
        *self.prepare.get(block_hash).unwrap()
    }

    pub fn commit(&mut self, block_hash: &str) -> bool {
        *self.commit.get_mut(block_hash).unwrap() += 1;
        *self.commit.get(block_hash).unwrap() >= 2 * self.max_faulty + 1
    }
}
