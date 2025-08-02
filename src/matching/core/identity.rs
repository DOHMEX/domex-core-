// ===============================
// identity.rs — Domex Poseidon Identity (Pasta Curve Only)
// ===============================

use pasta_curves::Fp;
use crate::poseidon_utils::recompute_delegation_hash;
use crate::vault_registry::get_owner_for_vault;
use crate::types::{VaultId, DelegationHash};

/// Verifies that the user-submitted Poseidon identity hash matches the registered vault owner.
pub fn verify_poseidon_auth(submitted_hash: &str, vault_id: &VaultId) -> bool {
    match get_owner_for_vault(vault_id) {
        Some(registered) => registered == *submitted_hash,
        None => false,
    }
}

/// Computes Poseidon(vault_id || delegate_pubkey) using Pasta Fp
pub fn compute_delegation_hash(vault_id: &VaultId, delegate_pubkey: &str) -> DelegationHash {
    recompute_delegation_hash(vault_id, delegate_pubkey)
}
