// ==========================================================
// delegator_fuel.rs — Domex Fuel Burn Tracker (Delegator-Paid Fees)
// ==========================================================
//
// Tracks proof submissions by delegators who burn fuel for execution.
// Used to bind vault activity to an authorized fuel-burning wallet.
// Allows attribution of gas usage, validator rewards, and replay checks.

use crate::poseidon_utils::bytes_to_goldilocks;
use plonky2::field::goldilocks_field::GoldilocksField;
use serde::{Serialize, Deserialize};

/// A delegator’s public key and fuel submission metadata.
/// This is attached to ZK proofs for validator-side replay and verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegatorFuelReceipt {
    pub delegator_pubkey: [u8; 32],  // Wallet that submitted proof + paid gas
    pub fuel_amount: Option<u64>,    // Optional gas value (e.g., satoshis or wei)
    pub timestamp: u64,              // UNIX timestamp of proof submission
}

/// Computes a delegator hash: Poseidon(delegate_pubkey)
///
/// This is used inside public inputs and validator-side replay checks.
pub fn compute_delegator_hash(pubkey: &[u8; 32]) -> GoldilocksField {
    bytes_to_goldilocks(pubkey)
}

/// Verifies if a delegator hash matches a provided public key.
pub fn verify_fuel_signature(
    expected: &GoldilocksField,
    delegate_pubkey: &[u8; 32],
) -> bool {
    let computed = compute_delegator_hash(delegate_pubkey);
    computed == *expected
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_delegator_hash_stability() {
        let pubkey = [0x11u8; 32];
        let h1 = compute_delegator_hash(&pubkey);
        let h2 = compute_delegator_hash(&pubkey);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_fuel_receipt_validity() {
        let pubkey = [0xABu8; 32];
        let expected = compute_delegator_hash(&pubkey);
        assert!(verify_fuel_signature(&expected, &pubkey));
    }

    #[test]
    fn test_fuel_receipt_struct() {
        let pubkey = [0x22u8; 32];
        let receipt = DelegatorFuelReceipt {
            delegator_pubkey: pubkey,
            fuel_amount: Some(1500),
            timestamp: Utc::now().timestamp() as u64,
        };
        assert_eq!(receipt.delegator_pubkey, pubkey);
        assert!(receipt.fuel_amount.is_some());
    }
}
