use std::collections::HashSet;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    pub tx_id: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub gas: u64,
    pub timestamp: u64,
}

pub struct TxPool {
    pending_txs: Vec<Transaction>,
    tx_ids: HashSet<String>,
    max_size: usize,
}

impl TxPool {
    pub fn new(max_size: usize) -> Self {
        TxPool {
            pending_txs: Vec::new(),
            tx_ids: HashSet::new(),
            max_size,
        }
    }

    pub fn add_transaction(&mut self, tx: Transaction) -> bool {
        if self.tx_ids.contains(&tx.tx_id) || self.pending_txs.len() >= self.max_size {
            return false;
        }
        self.tx_ids.insert(tx.tx_id.clone());
        self.pending_txs.push(tx);
        true
    }

    pub fn remove_transactions(&mut self, tx_ids: &[String]) {
        self.pending_txs.retain(|tx| !tx_ids.contains(&tx.tx_id));
        for id in tx_ids {
            self.tx_ids.remove(id);
        }
    }

    pub fn get_pending(&self, limit: usize) -> Vec<Transaction> {
        self.pending_txs.iter()
            .take(limit)
            .cloned()
            .collect()
    }

    pub fn clear(&mut self) {
        self.pending_txs.clear();
        self.tx_ids.clear();
    }
}
