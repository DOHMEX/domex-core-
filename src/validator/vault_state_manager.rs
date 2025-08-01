// Domex :: vault_state_manager.rs
// Tracks vault state for Poseidon-based vaults and generates Merkle leaf hashes

use bitcoin::Txid;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::poseidon_utils::poseidon_hash4;

/// Vault state tracked by Domex global validators
/// Used to compute Poseidon-based Merkle tree leaves
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultState {
    pub vault_id: String,              // e.g., "vault-btc-01"
    pub identity_hash: [u8; 32],       // Poseidon(sk || vault_id || zk_node_id)
    pub pool_hash: [u8; 32],           // Poseidon(script_bytes)
    pub balance_sat: u64,              // Confirmed vault balance
    pub last_updated: u64,             // UNIX timestamp
    pub last_txid: Option<Txid>,       // Optional BTC txid (for BTC-native vaults)
}

impl VaultState {
    /// Constructs a new vault entry
    pub fn new(
        vault_id: impl Into<String>,
        identity_hash: [u8; 32],
        pool_hash: [u8; 32],
        balance_sat: u64,
        last_txid: Option<Txid>,
    ) -> Self {
        Self {
            vault_id: vault_id.into(),
            identity_hash,
            pool_hash,
            balance_sat,
            last_updated: current_unix_timestamp(),
            last_txid,
        }
    }

    /// Apply a trade to the vault (buy or sell)
    /// Updates balance and timestamp — triggers Merkle root update at validator layer
    pub fn apply_trade(&mut self, delta_sat: i64) {
        let updated = (self.balance_sat as i64) + delta_sat;
        assert!(updated >= 0, "Trade underflow: vault balance below zero");
        self.balance_sat = updated as u64;
        self.last_updated = current_unix_timestamp();
    }

    /// Apply a confirmed deposit (e.g., BTC sent to vault script)
    pub fn apply_deposit(&mut self, amount: u64, txid: Txid) {
        self.balance_sat += amount;
        self.last_txid = Some(txid);
        self.last_updated = current_unix_timestamp();
    }

    /// Apply a withdrawal (burn operation) after validator proof confirmation
    pub fn apply_withdrawal(&mut self, amount: u64) {
        assert!(amount <= self.balance_sat, "Withdraw exceeds vault balance");
        self.balance_sat -= amount;
        self.last_updated = current_unix_timestamp();
    }

    /// Returns Poseidon-based Merkle leaf for this vault
    /// Format: Poseidon(identity_hash || pool_hash || balance || timestamp)
    pub fn to_merkle_leaf(&self) -> [u8; 32] {
        poseidon_hash4(
            &self.identity_hash,
            &self.pool_hash,
            self.balance_sat,
            self.last_updated,
        )
    }

    /// Checks if vault's leaf matches expected hash (validator-side validation)
    pub fn validate_merkle_leaf(&self, expected_leaf: [u8; 32]) -> bool {
        self.to_merkle_leaf() == expected_leaf
    }

    /// Serializes vault state to JSON (for logs or proofs)
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

/// Returns current UNIX time in seconds
fn current_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
