// Domex :: vault_state_manager.rs
// Tracks vault state for Poseidon-based Domex vaults and generates Merkle leaf hashes

use bitcoin::Txid;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::poseidon_utils::poseidon_hash4;

/// Vault state tracked by Domex global validators (applies to all vault types)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultState {
    pub vault_id: String,              // e.g., "vault-btc-01"
    pub identity_hash: [u8; 32],       // Poseidon(sk || vault_id || zk_node_id)
    pub pool_hash: [u8; 32],           // Poseidon(script_bytes)
    pub balance: u64,                  // Token balance (in sats, wei, or minimal unit)
    pub last_updated: u64,             // UNIX timestamp (seconds)
    pub last_txid: Option<Txid>,       // Optional: BTC last UTXO deposit
}

impl VaultState {
    /// Create a new vault snapshot with current timestamp
    pub fn new(
        vault_id: impl Into<String>,
        identity_hash: [u8; 32],
        pool_hash: [u8; 32],
        balance: u64,
        last_txid: Option<Txid>,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            vault_id: vault_id.into(),
            identity_hash,
            pool_hash,
            balance,
            last_updated: now,
            last_txid,
        }
    }

    /// Apply a trade to this vault (positive = buy, negative = sell)
    pub fn apply_trade(&mut self, delta: i64) {
        let updated = (self.balance as i64) + delta;
        assert!(updated >= 0, "Trade would underflow vault balance");
        self.balance = updated as u64;
        self.last_updated = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// Generate a Poseidon Merkle leaf (ZK-consumable)
    /// Merkle Leaf = Poseidon(identity_hash || pool_hash || balance || last_updated)
    pub fn to_merkle_leaf(&self) -> [u8; 32] {
        poseidon_hash4(
            &self.identity_hash,
            &self.pool_hash,
            self.balance,
            self.last_updated,
        )
    }

    /// Validate against a known Merkle leaf (for ZK state sync)
    pub fn validate_merkle_leaf(&self, expected_leaf: [u8; 32]) -> bool {
        self.to_merkle_leaf() == expected_leaf
    }

    /// Export vault state to JSON
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}
