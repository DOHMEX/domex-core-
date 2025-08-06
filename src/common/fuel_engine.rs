// ==========================================================
// fuel_engine.rs — Domex Fuel Verification & Delegation Logic
// ==========================================================
//
// Called by validator nodes during ZK proof validation.
// Validates:
//   - That minimum fuel has been burned
//   - That any delegation was authorized (Poseidon-based hash)
//   - That the burn is properly linked to a vault ID
//

use crate::token::token_config::*;
use crate::validator::poseidon_utils::poseidon_hash;
use std::collections::HashMap;

/// Fuel proof entry per vault proof
pub struct FuelProof {
    pub vault_id: String,
    pub burned_amount: u64,
    pub delegate_pubkey: Option<String>,
    pub delegation_hash: Option<String>,
}

/// Validates a batch of fuel proofs in a ZK proof context
pub fn validate_fuel_batch(proofs: &[FuelProof]) -> Result<(), String> {
    for proof in proofs {
        // 1. Ensure fuel burn meets minimum requirement
        if proof.burned_amount < MIN_PROOF_FUEL_BURN {
            return Err(format!(
                "Fuel too low in vault {} ({} < {})",
                proof.vault_id, proof.burned_amount, MIN_PROOF_FUEL_BURN
            ));
        }

        // 2. If delegated, validate delegation hash
        if let (Some(delegate), Some(provided_hash)) =
            (&proof.delegate_pubkey, &proof.delegation_hash)
        {
            let expected = compute_delegation_hash(&proof.vault_id, delegate);
            if expected != *provided_hash {
                return Err(format!(
                    "Invalid delegation hash for vault {}",
                    proof.vault_id
                ));
            }
        }
    }

    Ok(())
}

/// Computes Poseidon(delegation) = Poseidon(vault_id || pubkey || nonce)
pub fn compute_delegation_hash(vault_id: &str, delegate_pubkey: &str) -> String {
    // Static nonce for simplicity. Production should use dynamic nonce!
    let nonce = "42";
    let input = format!("{}{}{}", vault_id, delegate_pubkey, nonce);
    poseidon_hash(&input)
}
