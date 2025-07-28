// ===============================
// types/ownership.rs
// ===============================

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Tracks who owns what inside a vault (based on Poseidon hashes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipLedger {
    /// token → PoseidonHash → amount owned
    pub ownership: HashMap<String, HashMap<String, u64>>,
}

impl OwnershipLedger {
    /// Transfer vault ownership rights (e.g. after a trade)
    pub fn transfer(&mut self, token: &str, from: &str, to: &str, amount: u64) {
        // Decrease from current owner
        let from_map = self.ownership.entry(token.to_string()).or_default();
        let from_entry = from_map.entry(from.to_string()).or_insert(0);
        *from_entry = from_entry.saturating_sub(amount);

        // Increase for new owner
        let to_map = self.ownership.entry(token.to_string()).or_default();
        let to_entry = to_map.entry(to.to_string()).or_insert(0);
        *to_entry += amount;
    }

    /// Query how much a Poseidon identity owns of a token
    pub fn get_claimable(&self, token: &str, identity: &str) -> u64 {
        self.ownership
            .get(token)
            .and_then(|m| m.get(identity))
            .cloned()
            .unwrap_or(0)
    }
}
