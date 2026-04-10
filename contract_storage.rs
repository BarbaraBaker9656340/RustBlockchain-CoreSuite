use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractData {
    pub owner: String,
    pub code_hash: String,
    pub storage: HashMap<String, Vec<u8>>,
    pub create_time: u64,
}

pub struct ContractStorage {
    contracts: HashMap<String, ContractData>,
}

impl ContractStorage {
    pub fn new() -> Self {
        ContractStorage {
            contracts: HashMap::new(),
        }
    }

    pub fn deploy_contract(&mut self, addr: String, data: ContractData) -> bool {
        if self.contracts.contains_key(&addr) {
            return false;
        }
        self.contracts.insert(addr, data);
        true
    }

    pub fn get_contract(&self, addr: &str) -> Option<ContractData> {
        self.contracts.get(addr).cloned()
    }

    pub fn write_storage(&mut self, addr: &str, key: String, value: Vec<u8>) -> bool {
        if let Some(contract) = self.contracts.get_mut(addr) {
            contract.storage.insert(key, value);
            true
        } else {
            false
        }
    }

    pub fn read_storage(&self, addr: &str, key: &str) -> Option<Vec<u8>> {
        self.contracts.get(addr)?.storage.get(key).cloned()
    }
}
