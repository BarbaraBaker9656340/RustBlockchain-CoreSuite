use super::transaction_pool::{Transaction, TxPool};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct MemPoolOptimizer {
    max_gas_price: u64,
    min_gas_limit: u64,
}

impl MemPoolOptimizer {
    pub fn new(max_gas: u64, min_gas: u64) -> Self {
        MemPoolOptimizer {
            max_gas_price: max_gas,
            min_gas_limit: min_gas,
        }
    }

    pub fn filter_transactions(&self, pool: &TxPool) -> Vec<Transaction> {
        let mut heap = BinaryHeap::new();
        for tx in pool.get_pending(1000) {
            if tx.gas >= self.min_gas_limit && tx.gas <= self.max_gas_price {
                heap.push(Reverse((tx.gas, tx)));
            }
        }
        heap.into_sorted_vec().into_iter().map(|r| r.1.1).collect()
    }

    pub fn clean_low_priority(&self, pool: &mut TxPool) {
        let to_remove: Vec<String> = pool.get_pending(1000)
            .into_iter()
            .filter(|tx| tx.gas < self.min_gas_limit)
            .map(|tx| tx.tx_id)
            .collect();
        pool.remove_transactions(&to_remove);
    }
}
