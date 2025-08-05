// ==================================================================
// delegation_contract.rs — Domex Delegator Fee Agreement (with Payload)
// ==================================================================
//
// Defines off-chain delegator rules for fee collection and vault servicing.
// This metadata is signed off-chain and used by clients to pick a delegator.
//
// Includes a flexible `payload` field to embed:
// - expiry timestamp
// - supported tokens list
// - regional/geographic filters
// - refund conditions
// - DAO-style governance options

use serde::{Serialize, Deserialize};
use crate::types::common::Token;

/// Payment method expected by the delegator
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FeeMode {
    Percentage(f64), // % of vault (e.g., 0.5 means 0.5%)
    Flat(f64),       // Fixed amount in `accepted_token`
    Free,            // Zero fee — e.g., for testnet or promo
}

/// Main delegator contract struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationContract {
    pub delegator_pubkey: [u8; 32], // Fuel submitter
    pub accepted_token: Token,      // Default fee currency (e.g., USDT)
    pub fee_mode: FeeMode,          // Fee calculation logic
    pub min_vault_amount: Option<f64>, // Optional minimum vault size
    pub metadata_uri: Option<String>,  // External info or contact
    pub payload: Option<String>,       // Custom JSON-encoded data
}

impl DelegationContract {
    /// Calculate fee based on vault size
    pub fn calculate_fee(&self, vault_amount: f64) -> f64 {
        match self.fee_mode {
            FeeMode::Percentage(p) => (vault_amount * p) / 100.0,
            FeeMode::Flat(fee) => fee,
            FeeMode::Free => 0.0,
        }
    }

    /// Check if vault size meets minimum
    pub fn is_vault_acceptable(&self, amount: f64) -> bool {
        match self.min_vault_amount {
            Some(min) => amount >= min,
            None => true,
        }
    }
}
