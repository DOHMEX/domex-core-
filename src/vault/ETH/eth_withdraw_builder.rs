// Domex :: eth_withdraw_builder.rs
// ETH withdrawal intent builder for Poseidon-bound Domex vaults

use ethers::types::Address;
use crate::poseidon_utils::poseidon_hash4;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a structured ETH withdrawal intent for Domex validators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthWithdrawIntent {
    pub vault_id: String,
    pub eth_address: Address,
    pub amount_wei: u128,
    pub timestamp: u64,
    pub intent_hash: [u8; 32], // Poseidon(identity_hash || amount || timestamp || eth_address)
}

impl EthWithdrawIntent {
    /// Creates a new withdrawal intent and computes its Poseidon hash
    pub fn new(
        vault_id: impl Into<String>,
        eth_address: Address,
        identity_hash: [u8; 32],
        amount_wei: u128,
    ) -> Self {
        let now = current_unix_timestamp();

        let hash = poseidon_hash4(
            &identity_hash,
            &eth_address.as_bytes(),
            amount_wei,
            now,
        );

        Self {
            vault_id: vault_id.into(),
            eth_address,
            amount_wei,
            timestamp: now,
            intent_hash: hash,
        }
    }

    /// Returns the Poseidon-based hash of the withdrawal intent
    pub fn to_hash(&self) -> [u8; 32] {
        self.intent_hash
    }

    /// Serialize intent for submission or logging
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

/// Returns current UNIX timestamp in seconds
fn current_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
