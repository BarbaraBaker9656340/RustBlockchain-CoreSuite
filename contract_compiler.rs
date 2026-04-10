use super::smart_vm_engine::VMOpcode;

#[derive(Debug, Clone)]
pub struct ContractSource {
    pub code: String,
    pub contract_name: String,
}

pub struct ContractCompiler;

impl ContractCompiler {
    pub fn compile(source: &ContractSource) -> Result<Vec<VMOpcode>, String> {
        let mut opcodes = Vec::new();
        let tokens: Vec<&str> = source.code.split_whitespace().collect();

        for token in tokens {
            match token {
                "push" => continue,
                "add" => opcodes.push(VMOpcode::Add),
                "sub" => opcodes.push(VMOpcode::Sub),
                "mul" => opcodes.push(VMOpcode::Mul),
                "store" => opcodes.push(VMOpcode::Store),
                "load" => opcodes.push(VMOpcode::Load),
                num if num.parse::<u64>().is_ok() => {
                    let val = num.parse::<u64>().unwrap();
                    opcodes.push(VMOpcode::Push(val));
                }
                _ => return Err(format!("未知指令: {}", token)),
            }
        }
        Ok(opcodes)
    }

    pub fn validate_syntax(source: &ContractSource) -> bool {
        !source.code.is_empty() && !source.contract_name.is_empty()
    }
}
