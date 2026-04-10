use super::blockchain_core::{Block, Blockchain};
use super::state_database::StateDB;

pub struct ChainBootstrap {
    chain_id: String,
    genesis_alloc: HashMap<String, u64>,
}

impl ChainBootstrap {
    pub fn new(chain_id: &str) -> Self {
        ChainBootstrap {
            chain_id: chain_id.to_string(),
            genesis_alloc: HashMap::new(),
        }
    }

    pub fn add_genesis_account(&mut self, addr: String, balance: u64) {
        self.genesis_alloc.insert(addr, balance);
    }

    pub fn build_genesis(&self) -> (Blockchain, StateDB) {
        let mut chain = Blockchain::new();
        let mut state = StateDB::new();

        for (addr, balance) in &self.genesis_alloc {
            let account = super::state_database::AccountState {
                balance: *balance,
                nonce: 0,
                contract_code: Vec::new(),
                storage: HashMap::new(),
            };
            state.update_account(addr.clone(), account);
        }

        (chain, state)
    }
}
