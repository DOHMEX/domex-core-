// ==========================================================
// balance_snapshot.rs — Immutable Token Balance Snapshot
// ==========================================================
//
// Records and verifies historical DOMEX token balances
// for voting, reward distribution, or validator eligibility.
//

use std::collections::HashMap;

/// A snapshot of user token balances at a given block height.
#[derive(Debug, Clone)]
pub struct BalanceSnapshot {
    pub block_height: u64,
    pub balances: HashMap<String, u64>, // Key: wallet address (as hex string)
}

impl BalanceSnapshot {
    /// Creates a new snapshot at a given height
    pub fn new(block_height: u64, balances: HashMap<String, u64>) -> Self {
        BalanceSnapshot {
            block_height,
            balances,
        }
    }

    /// Gets the balance of a specific user (returns 0 if not found)
    pub fn get_balance(&self, user_address: &str) -> u64 {
        *self.balances.get(user_address).unwrap_or(&0)
    }

    /// Returns total supply represented in this snapshot
    pub fn total_supply(&self) -> u64 {
        self.balances.values().sum()
    }

    /// Returns a Merkle-style hash for the snapshot for proof-of-inclusion (optional)
    pub fn snapshot_hash(&self) -> [u8; 32] {
        use crate::poseidon_utils::poseidon_hash;

        // Flatten: [address, balance] bytes for all users
        let mut all_data = vec![];
        for (addr, bal) in self.balances.iter() {
            all_data.extend_from_slice(addr.as_bytes());
            all_data.extend_from_slice(&bal.to_be_bytes());
        }

        poseidon_hash(&all_data)
    }
}
