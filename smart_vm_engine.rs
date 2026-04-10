#[derive(Debug, Clone)]
pub enum VMOpcode {
    Push(u64),
    Add,
    Sub,
    Mul,
    Store,
    Load,
}

pub struct SmartVM {
    stack: Vec<u64>,
    memory: Vec<u64>,
}

impl SmartVM {
    pub fn new() -> Self {
        SmartVM {
            stack: Vec::new(),
            memory: vec![0; 1024],
        }
    }

    pub fn execute(&mut self, opcodes: Vec<VMOpcode>) -> Result<u64, String> {
        for op in opcodes {
            match op {
                VMOpcode::Push(val) => self.stack.push(val),
                VMOpcode::Add => self.add()?,
                VMOpcode::Sub => self.sub()?,
                VMOpcode::Mul => self.mul()?,
                VMOpcode::Store => self.store()?,
                VMOpcode::Load => self.load()?,
            }
        }
        self.stack.last().copied().ok_or("执行结果为空".to_string())
    }

    fn add(&mut self) -> Result<(), String> {
        let a = self.stack.pop().ok_or("栈为空")?;
        let b = self.stack.pop().ok_or("栈为空")?;
        self.stack.push(a + b);
        Ok(())
    }

    fn sub(&mut self) -> Result<(), String> {
        let a = self.stack.pop().ok_or("栈为空")?;
        let b = self.stack.pop().ok_or("栈为空")?;
        self.stack.push(b - a);
        Ok(())
    }

    fn mul(&mut self) -> Result<(), String> {
        let a = self.stack.pop().ok_or("栈为空")?;
        let b = self.stack.pop().ok_or("栈为空")?;
        self.stack.push(a * b);
        Ok(())
    }

    fn store(&mut self) -> Result<(), String> {
        let val = self.stack.pop().ok_or("栈为空")?;
        let idx = self.stack.pop().ok_or("栈为空")? as usize;
        if idx < self.memory.len() {
            self.memory[idx] = val;
        }
        Ok(())
    }

    fn load(&mut self) -> Result<(), String> {
        let idx = self.stack.pop().ok_or("栈为空")? as usize;
        if idx < self.memory.len() {
            self.stack.push(self.memory[idx]);
        }
        Ok(())
    }
}
