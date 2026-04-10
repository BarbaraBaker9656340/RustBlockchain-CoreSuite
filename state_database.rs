use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountState {
    pub balance: u64,
    pub nonce: u64,
    pub contract_code: Vec<u8>,
    pub storage: HashMap<String, String>,
}

pub struct StateDB {
    state: HashMap<String, AccountState>,
    snapshot: HashMap<u64, HashMap<String, AccountState>>,
    current_height: u64,
}

impl StateDB {
    pub fn new() -> Self {
        StateDB {
            state: HashMap::new(),
            snapshot: HashMap::new(),
            current_height: 0,
        }
    }

    pub fn get_account(&self, addr: &str) -> AccountState {
        self.state.get(addr).cloned().unwrap_or_else(|| AccountState {
            balance: 0,
            nonce: 0,
            contract_code: Vec::new(),
            storage: HashMap::new(),
        })
    }

    pub fn update_account(&mut self, addr: String, state: AccountState) {
        self.state.insert(addr, state);
    }

    pub fn create_snapshot(&mut self) {
        self.snapshot.insert(self.current_height, self.state.clone());
        self.current_height += 1;
    }

    pub fn rollback(&mut self, height: u64) -> bool {
        if let Some(snap) = self.snapshot.get(&height) {
            self.state = snap.clone();
            self.current_height = height;
            true
        } else {
            false
        }
    }
}
