use super::smart_vm_engine::VMOpcode;

pub struct ContractAuditor;

impl ContractAuditor {
    pub fn audit_security(opcodes: &[VMOpcode]) -> Vec<String> {
        let mut issues = Vec::new();
        let mut stack_depth = 0;

        for op in opcodes {
            match op {
                VMOpcode::Push(_) => stack_depth += 1,
                VMOpcode::Add | VMOpcode::Sub | VMOpcode::Mul => stack_depth -= 1,
                _ => {}
            }
            if stack_depth > 1024 {
                issues.push("栈溢出风险".to_string());
            }
        }

        if !Self::check_storage_safe(opcodes) {
            issues.push("存储操作未授权风险".to_string());
        }

        issues
    }

    fn check_storage_safe(opcodes: &[VMOpcode]) -> bool {
        let store_count = opcodes.iter().filter(|op| matches!(op, VMOpcode::Store)).count();
        let load_count = opcodes.iter().filter(|op| matches!(op, VMOpcode::Load)).count();
        store_count <= load_count
    }
}
