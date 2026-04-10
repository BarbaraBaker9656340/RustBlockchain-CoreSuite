use std::collections::HashMap;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct FungibleToken {
    pub name: String,
    pub symbol: String,
    pub total_supply: u64,
    balances: HashMap<String, u64>,
    allowances: HashMap<String, HashMap<String, u64>>,
}

impl FungibleToken {
    pub fn new(name: &str, symbol: &str, total_supply: u64, owner: &str) -> Self {
        let mut balances = HashMap::new();
        balances.insert(owner.to_string(), total_supply);
        
        FungibleToken {
            name: name.to_string(),
            symbol: symbol.to_string(),
            total_supply,
            balances,
            allowances: HashMap::new(),
        }
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> bool {
        let from_bal = *self.balances.get(from).unwrap_or(&0);
        if from_bal < amount {
            return false;
        }
        *self.balances.get_mut(from).unwrap() -= amount;
        *self.balances.entry(to.to_string()).or_insert(0) += amount;
        true
    }
}
