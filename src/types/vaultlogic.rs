// types/vaultlogic.rs — Vault-specific types

#[derive(Clone)]
pub struct OrderInstruction {
    pub owner_hash: String,
    pub counterparty_hash: String,
    pub token: String,
    pub price: u64,
    pub size: u64,
}

#[derive(Clone)]
pub struct TradeResult {
    pub vault_id: String,
    pub executed_price: u64,
    pub buyer: String,
    pub seller: String,
    pub token: String,
    pub size: u64,
    pub balance_delta: Vec<BalanceChange>,
}

#[derive(Clone)]
pub struct BalanceChange {
    pub identity: String,
    pub token: String,
    pub delta: i64,
}

#[derive(Clone)]
pub struct VaultState {
    pub vault_id: String,
    pub balances: std::collections::HashMap<(String, String), u64>,
}

impl VaultState {
    pub fn get_balance(&self, identity: &str, token: &str) -> u64 {
        *self.balances.get(&(identity.to_string(), token.to_string())).unwrap_or(&0)
    }

    pub fn decrease_balance(&mut self, identity: &str, token: &str, amount: u64) {
        let key = (identity.to_string(), token.to_string());
        let entry = self.balances.entry(key).or_insert(0);
        *entry = entry.saturating_sub(amount);
    }

    pub fn increase_balance(&mut self, identity: &str, token: &str, amount: u64) {
        let key = (identity.to_string(), token.to_string());
        let entry = self.balances.entry(key).or_insert(0);
        *entry += amount;
    }
}
