// ==========================================================
// token_attestation.rs — Validator Attestation for Token Proofs
// ==========================================================
//
// Generates Poseidon-hashed attestations for DOMEX token-related
// state transitions (mint, withdraw, fuel burn, recycle).
// Used in the validator consensus pipeline.
//

use crate::token_config::*;
use crate::token_state::TokenVaultState;
use poseidon_utils::poseidon_hash;

/// Represents a signed token state attestation.
#[derive(Debug, Clone)]
pub struct TokenAttestation {
    pub vault_id: String,
    pub prev_root: [u8; 32],
    pub new_root: [u8; 32],
    pub total_minted: u64,
    pub total_burned: u64,
    pub fuel_recycled: u64,
    pub block_height: u64,
}

impl TokenAttestation {
    /// Create a new attestation from token vault state.
    pub fn from_vault_state(
        vault_id: &str,
        prev: &TokenVaultState,
        new: &TokenVaultState,
        block_height: u64,
    ) -> Self {
        TokenAttestation {
            vault_id: vault_id.to_string(),
            prev_root: prev.merkle_root,
            new_root: new.merkle_root,
            total_minted: new.total_minted - prev.total_minted,
            total_burned: new.total_burned - prev.total_burned,
            fuel_recycled: recycled_fuel_amount(new.total_burned - prev.total_burned),
            block_height,
        }
    }

    /// Computes Poseidon hash root to be signed by validators.
    pub fn to_attest_hash(&self) -> [u8; 32] {
        poseidon_hash(&[
            self.vault_id.as_bytes(),
            &self.prev_root,
            &self.new_root,
            &self.total_minted.to_be_bytes(),
            &self.total_burned.to_be_bytes(),
            &self.fuel_recycled.to_be_bytes(),
            &self.block_height.to_be_bytes(),
        ])
    }
}
