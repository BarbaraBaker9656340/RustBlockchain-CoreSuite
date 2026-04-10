use super::transaction_pool::Transaction;
use super::smart_vm_engine::VMOpcode;

pub struct GasCalculator {
    base_gas: u64,
    op_gas: u64,
    storage_gas: u64,
}

impl GasCalculator {
    pub fn new() -> Self {
        GasCalculator {
            base_gas: 21000,
            op_gas: 100,
            storage_gas: 20000,
        }
    }

    pub fn calculate_tx_gas(&self, tx: &Transaction) -> u64 {
        self.base_gas + tx.gas
    }

    pub fn calculate_contract_gas(&self, opcodes: &[VMOpcode]) -> u64 {
        let mut gas = 0;
        for op in opcodes {
            match op {
                VMOpcode::Push(_) => gas += self.op_gas,
                VMOpcode::Add | VMOpcode::Sub | VMOpcode::Mul => gas += self.op_gas,
                VMOpcode::Store | VMOpcode::Load => gas += self.storage_gas,
            }
        }
        gas
    }

    pub fn is_gas_sufficient(&self, tx: &Transaction, required: u64) -> bool {
        tx.gas >= required
    }
}
