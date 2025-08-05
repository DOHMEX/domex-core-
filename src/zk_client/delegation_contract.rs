// ================================================================
// delegation_contract.rs — Domex Delegator Fee Agreement (Off-Chain JSON)
// ================================================================
//
// Defines off-chain fee expectations and conditions for delegators
// who burn fuel to submit proofs on behalf of users.
//
// This does NOT execute on-chain — it is a verifiable JSON-based
// metadata format that clients may agree to and validators may reference.

use serde::{Serialize, Deserialize};
use crate::types::common::{Token};

/// Payment mode the delegator expects for providing ZK proof services
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FeeMode {
    Percentage(f64), // e.g. 0.5 means 0.5% of deposited amount
    Flat(f64),       // Fixed fee per proof, denominated in `token`
    Free,            // No fee (e.g., subsidized or testnet)
}

/// Delegator contract describing fee and preferred payment method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationContract {
    pub delegator_pubkey: [u8; 32], // Identity of the fuel provider
    pub accepted_token: Token,      // What token to pay fee in (e.g., USDT)
    pub fee_mode: FeeMode,          // % or flat fee
    pub min_vault_amount: Option<f64>, // Optional min vault size for proof acceptance
    pub metadata_uri: Option<String>,  // Optional URI for T&C or contact info
}

impl DelegationContract {
    /// Returns the fee amount based on the contract settings
    pub fn calculate_fee(&self, vault_amount: f64) -> f64 {
        match self.fee_mode {
            FeeMode::Percentage(pct) => (vault_amount * pct) / 100.0,
            FeeMode::Flat(fee) => fee,
            FeeMode::Free => 0.0,
        }
    }

    /// Validates whether a vault amount is acceptable for this contract
    pub fn is_vault_acceptable(&self, amount: f64) -> bool {
        match self.min_vault_amount {
            Some(min) => amount >= min,
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentage_fee() {
        let contract = DelegationContract {
            delegator_pubkey: [0xAB; 32],
            accepted_token: "USDT".to_string(),
            fee_mode: FeeMode::Percentage(1.5),
            min_vault_amount: Some(50.0),
            metadata_uri: None,
        };

        assert_eq!(contract.calculate_fee(200.0), 3.0);
        assert!(contract.is_vault_acceptable(100.0));
        assert!(!contract.is_vault_acceptable(40.0));
    }

    #[test]
    fn test_flat_fee() {
        let contract = DelegationContract {
            delegator_pubkey: [0xCD; 32],
            accepted_token: "ETH".to_string(),
            fee_mode: FeeMode::Flat(0.002),
            min_vault_amount: None,
            metadata_uri: None,
        };

        assert_eq!(contract.calculate_fee(1.5), 0.002);
        assert!(contract.is_vault_acceptable(0.1));
    }
}
