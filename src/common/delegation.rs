// ==========================================================================
// delegation.rs — Domex Delegation Logic for Fuel Sponsorship (GitHub-ready)
// ==========================================================================
//
// Vault owners can delegate fuel-paying authority to third parties.
// This file defines:
// - Poseidon-hashed delegation structures
// - Nonce-based replay protection
// - Verification and serialization logic
//

use serde::{Deserialize, Serialize};
use crate::common::poseidon_utils::poseidon_hash3;

/// Represents a delegation grant from a vault owner to a fuel sponsor
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Delegation {
    pub vault_id: [u8; 32],          // Poseidon hash of vault owner's identity
    pub delegate_pubkey: [u8; 32],   // Authorized third-party key
    pub nonce: u64,                  // Prevents replay attacks
}

impl Delegation {
    /// Computes the Poseidon hash of the delegation (used in proof attestation)
    pub fn to_poseidon_hash(&self) -> [u8; 32] {
        poseidon_hash3(self.vault_id, self.delegate_pubkey, self.nonce)
    }

    /// Checks if two delegations are logically equivalent
    pub fn matches(&self, other: &Delegation) -> bool {
        self.vault_id == other.vault_id
            && self.delegate_pubkey == other.delegate_pubkey
            && self.nonce == other.nonce
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delegation_hash_consistency() {
        let delegation = Delegation {
            vault_id: [1u8; 32],
            delegate_pubkey: [2u8; 32],
            nonce: 42,
        };

        let hash1 = delegation.to_poseidon_hash
