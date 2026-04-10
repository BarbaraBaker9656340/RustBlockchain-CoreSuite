use super::state_database::{StateDB, AccountState};
use std::collections::HashSet;

pub struct AccountManager {
    admin_accounts: HashSet<String>,
    frozen_accounts: HashSet<String>,
}

impl AccountManager {
    pub fn new() -> Self {
        AccountManager {
            admin_accounts: HashSet::new(),
            frozen_accounts: HashSet::new(),
        }
    }

    pub fn add_admin(&mut self, addr: String) {
        self.admin_accounts.insert(addr);
    }

    pub fn is_admin(&self, addr: &str) -> bool {
        self.admin_accounts.contains(addr)
    }

    pub fn freeze_account(&mut self, addr: String) {
        self.frozen_accounts.insert(addr);
    }

    pub fn unfreeze_account(&mut self, addr: &str) {
        self.frozen_accounts.remove(addr);
    }

    pub fn is_allowed(&self, addr: &str, state: &AccountState) -> bool {
        !self.frozen_accounts.contains(addr) && state.balance > 0
    }
}
