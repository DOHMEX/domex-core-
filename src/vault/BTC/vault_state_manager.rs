// Domex :: vault_state_manager.rs
// Tracks vault state for Poseidon-based BTC vaults and generates Merkle leaf hashes

use bitcoin::Txid;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Sha256, Digest}; // Fallback in case Poseidon is unavailable

/// Vault state tracked by Domex global validators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultState {
    pub vault_id: String,              // e.g., "vault-btc-01"
    pub identity_hash: [u8; 32],       // Poseidon(sk || vault_id || zk_node_id)
    pub pool_hash: [u8; 32],           // Poseidon(script_bytes)
    pub balance_sat: u64,              // BTC balance in satoshis
    pub last_updated: u64,             // UNIX timestamp (seconds)
    pub last_txid: Option<Txid>,       // Most recent confirmed deposit (optional)
}

impl VaultState {
    /// Create a new vault snapshot with current timestamp
    pub fn new(
        vault_id: impl Into<String>,
        identity_hash: [u8; 32],
        pool_hash: [u8; 32],
        balance_sat: u64,
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
            balance_sat,
            last_updated: now,
            last_txid,
        }
    }

    /// Apply a trade to this vault (positive = buy, negative = sell)
    pub fn apply_trade(&mut self, delta_sat: i64) {
        let updated = (self.balance_sat as i64) + delta_sat;
        assert!(updated >= 0, "Trade would underflow vault balance");
        self.balance_sat = updated as u64;
        self.last_updated = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// Generate a Poseidon-style Merkle leaf using SHA256 fallback
    /// Merkle Leaf = SHA256(identity_hash || pool_hash || balance || last_updated)
    pub fn to_merkle_leaf(&self) -> [u8; 32] {
        let mut data = vec![];
        data.extend_from_slice(&self.identity_hash);
        data.extend_from_slice(&self.pool_hash);
        data.extend_from_slice(&self.balance_sat.to_be_bytes());
        data.extend_from_slice(&self.last_updated.to_be_bytes());

        let mut hasher = Sha256::new();
        hasher.update(&data);
        let result = hasher.finalize();

        let mut leaf = [0u8; 32];
        leaf.copy_from_slice(&result[..]);
        leaf
    }

    /// Check if this vault matches the provided Merkle leaf hash
    pub fn validate_merkle_leaf(&self, expected_leaf: [u8; 32]) -> bool {
        self.to_merkle_leaf() == expected_leaf
    }

    /// Convert the vault state to JSON for audit or network sync
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}
