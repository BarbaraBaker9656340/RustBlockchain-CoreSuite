use serde::Serialize;
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize)]
pub struct CrossChainTx {
    pub tx_id: String,
    pub source_chain: String,
    pub target_chain: String,
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub status: CrossChainStatus,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum CrossChainStatus {
    Pending,
    Verified,
    Executed,
    Failed,
}

pub struct CrossChainBridge {
    pending_requests: VecDeque<CrossChainTx>,
    verified_txs: Vec<CrossChainTx>,
    chain_ids: Vec<String>,
}

impl CrossChainBridge {
    pub fn new() -> Self {
        CrossChainBridge {
            pending_requests: VecDeque::new(),
            verified_txs: Vec::new(),
            chain_ids: vec!["ETH".to_string(), "BSC".to_string(), "SOL".to_string()],
        }
    }

    pub fn create_cross_tx(&mut self, tx: CrossChainTx) -> bool {
        if !self.chain_ids.contains(&tx.source_chain) || !self.chain_ids.contains(&tx.target_chain) {
            return false;
        }
        self.pending_requests.push_back(tx);
        true
    }

    pub fn verify_next(&mut self) -> Option<CrossChainTx> {
        let mut tx = self.pending_requests.pop_front()?;
        tx.status = CrossChainStatus::Verified;
        self.verified_txs.push(tx.clone());
        Some(tx)
    }
}
