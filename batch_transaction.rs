use super::transaction_pool::Transaction;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct BatchTx {
    pub batch_id: String,
    pub transactions: Vec<Transaction>,
    pub total_gas: u64,
    pub timestamp: u64,
}

pub struct BatchTxProcessor;

impl BatchTxProcessor {
    pub fn create_batch(transactions: Vec<Transaction>) -> BatchTx {
        let total_gas = transactions.iter().map(|tx| tx.gas).sum();
        let timestamp = chrono::Utc::now().timestamp_millis() as u64;
        let batch_id = hex::encode(sha2::Sha256::digest(format!("{:?}{}", transactions, timestamp).as_bytes()));
        
        BatchTx {
            batch_id,
            transactions,
            total_gas,
            timestamp,
        }
    }

    pub fn validate_batch(batch: &BatchTx) -> bool {
        !batch.transactions.is_empty() && batch.total_gas > 0
    }

    pub fn unpack_batch(batch: BatchTx) -> Vec<Transaction> {
        batch.transactions
    }
}
