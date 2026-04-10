use super::blockchain_core::{Block, Blockchain};
use std::collections::HashMap;

pub struct ForkHandler {
    fork_chains: HashMap<u64, Vec<Block>>,
    main_chain_height: u64,
}

impl ForkHandler {
    pub fn new() -> Self {
        ForkHandler {
            fork_chains: HashMap::new(),
            main_chain_height: 0,
        }
    }

    pub fn detect_fork(&mut self, main_chain: &Blockchain, new_block: &Block) -> bool {
        let last = main_chain.chain.last().unwrap();
        if new_block.prev_hash != last.hash && new_block.index == last.index + 1 {
            self.fork_chains.entry(new_block.index).or_insert_with(Vec::new).push(new_block.clone());
            return true;
        }
        false
    }

    pub fn resolve_fork(&self, main_chain: &mut Blockchain) -> bool {
        for (height, blocks) in &self.fork_chains {
            if blocks.len() > main_chain.chain.len() - *height as usize {
                main_chain.chain.truncate(*height as usize);
                return true;
            }
        }
        false
    }
}
