// types/vault_raft_adapter.rs

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// A single vault’s current state (balances, vault ID, price)
#[derive(Clone, Debug)]
pub struct VaultState {
    pub vault_id: String,
    pub balances: HashMap<String, HashMap<String, u64>>, // identity → token → balance
}

impl VaultState {
    pub fn get_balance(&self, identity: &str, token: &str) -> u64 {
        *self.balances
            .get(identity)
            .and_then(|m| m.get(token))
            .unwrap_or(&0)
    }

    pub fn decrease_balance(&mut self, identity: &str, token: &str, amount: u64) {
        if let Some(user_balances) = self.balances.get_mut(identity) {
            let entry = user_balances.entry(token.to_string()).or_insert(0);
            *entry = entry.saturating_sub(amount);
        }
    }

    pub fn increase_balance(&mut self, identity: &str, token: &str, amount: u64) {
        self.balances
            .entry(identity.to_string())
            .or_default()
            .entry(token.to_string())
            .and_modify(|e| *e += amount)
            .or_insert(amount);
    }
}

/// A user's submitted or matched order instruction
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderInstruction {
    pub owner_hash: String,
    pub counterparty_hash: String,
    pub token: String,
    pub price: u64,
    pub size: u64,
}

/// Configuration and rules tied to a specific vault
#[derive(Clone, Debug)]
pub struct VaultMetadata {
    pub tick_size: u64,
    pub max_delta_bps: u64, // e.g. 200 = 2% delta rule
}

/// Result after executing a single trade
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeResult {
    pub vault_id: String,
    pub buyer: String,
    pub seller: String,
    pub token: String,
    pub size: u64,
    pub executed_price: u64,
    pub balance_delta: Vec<BalanceChange>,
}

/// Used to build Merkle tree delta
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BalanceChange {
    pub identity: String,
    pub token: String,
    pub delta: i64,
}
