// Domex :: eth_vault.rs
// ETH vault logic using Poseidon-based identity binding and balance tracking

use serde::{Deserialize, Serialize};
use ethers::types::H160;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::poseidon_utils::poseidon_hash2;

/// Represents a non-custodial ETH vault tracked by Domex
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthVault {
    pub vault_id: String,            // e.g., "vault-eth-01"
    pub identity_hash: [u8; 32],     // Poseidon(sk || vault_id)
    pub script_hash: [u8; 32],       // Poseidon(deposit_script_bytes)
    pub eth_address: H160,           // ETH L1 receiving address (script-derived)
    pub balance_wei: u128,           // ETH balance in wei
    pub last_updated: u64,           // UNIX timestamp
}

impl EthVault {
    /// Create a new ETH vault with Poseidon binding
    pub fn new(
        vault_id: impl Into<String>,
        private_key_bytes: &[u8],
        deposit_script_bytes: &[u8],
        eth_address: H160,
        initial_balance: u128,
    ) -> Self {
        let vault_id_str = vault_id.into();
        let identity_hash = poseidon_hash2(private_key_bytes, vault_id_str.as_bytes());
        let script_hash = poseidon_hash2(deposit_script_bytes, b"eth");

        Self {
            vault_id: vault_id_str,
            identity_hash,
            script_hash,
            eth_address,
            balance_wei: initial_balance,
            last_updated: current_unix_timestamp(),
        }
    }

    /// Apply a deposit to the ETH vault
    pub fn apply_deposit(&mut self, amount: u128) {
        self.balance_wei += amount;
        self.last_updated = current_unix_timestamp();
    }

    /// Apply a withdrawal to the ETH vault
    pub fn apply_withdrawal(&mut self, amount: u128) {
        assert!(amount <= self.balance_wei, "Withdraw exceeds vault balance");
        self.balance_wei -= amount;
        self.last_updated = current_unix_timestamp();
    }

    /// Return a Poseidon-style Merkle leaf for vault tracking
    pub fn to_merkle_leaf(&self) -> [u8; 32] {
        use crate::poseidon_utils::poseidon_hash4_u128;
        poseidon_hash4_u128(
            &self.identity_hash,
            &self.script_hash,
            self.balance_wei,
            self.last_updated,
        )
    }

    /// Validate Merkle leaf match
    pub fn validate_leaf(&self, expected_leaf: [u8; 32]) -> bool {
        self.to_merkle_leaf() == expected_leaf
    }

    /// Return JSON serialization (for audit or proof packaging)
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

/// Returns current UNIX timestamp (seconds)
fn current_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
