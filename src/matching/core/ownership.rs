// ===============================
// ownership.rs — Domex Post-Trade Identity Logic
// ===============================

use std::collections::HashMap;

/// Represents the internal claim ownership state of a vault
#[derive(Debug, Clone)]
pub struct OwnershipLedger {
    /// Mapping of token → Poseidon identity → claim size
    pub ownership: HashMap<String, HashMap<String, u64>>,
}

impl OwnershipLedger {
    /// Transfer ownership of a token amount from one Poseidon identity to another
    pub fn transfer(
        &mut self,
        token: &str,
        from: &str,
        to: &str,
        amount: u64,
    ) {
        // Subtract from current owner
        let from_map = self.ownership.entry(token.to_string()).or_default();
        let from_balance = from_map.entry(from.to_string()).or_insert(0);
        *from_balance = from_balance.saturating_sub(amount);

        // Add to new owner
        let to_map = self.ownership.entry(token.to_string()).or_default();
        let to_balance = to_map.entry(to.to_string()).or_insert(0);
        *to_balance += amount;
    }

    /// Get total claimable size for an identity and token
    pub fn get_claimable(&self, token: &str, identity: &str) -> u64 {
        self.ownership
            .get(token)
            .and_then(|m| m.get(identity))
            .cloned()
            .unwrap_or(0)
    }
}
