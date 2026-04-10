use std::collections::HashMap;

pub struct StakingPool {
    stakers: HashMap<String, u64>,
    total_staked: u64,
    reward_rate: f64,
}

impl StakingPool {
    pub fn new(reward_rate: f64) -> Self {
        StakingPool {
            stakers: HashMap::new(),
            total_staked: 0,
            reward_rate,
        }
    }

    pub fn stake(&mut self, addr: String, amount: u64) {
        *self.stakers.entry(addr).or_insert(0) += amount;
        self.total_staked += amount;
    }

    pub fn unstake(&mut self, addr: &str, amount: u64) -> bool {
        if let Some(balance) = self.stakers.get_mut(addr) {
            if *balance >= amount {
                *balance -= amount;
                self.total_staked -= amount;
                return true;
            }
        }
        false
    }

    pub fn calculate_reward(&self, addr: &str) -> u64 {
        let balance = *self.stakers.get(addr).unwrap_or(&0);
        (balance as f64 * self.reward_rate) as u64
    }
}
