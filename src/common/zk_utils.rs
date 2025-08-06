// ==========================================================
// zk_utils.rs — Domex ZK Helper Functions and Poseidon Inputs
// ==========================================================
//
// Utility functions for zero-knowledge proof preprocessing,
// input formatting, and sanity checks for the proof circuit.
// Used by validators, proof builders, and batch verifiers.
//

use crate::poseidon_utils::poseidon_hash;
use crate::common_types::{VaultId, MerkleRoot};

/// Formats ZK circuit inputs from Merkle roots and vault IDs
pub fn format_zk_inputs(
    vault_id: &VaultId,
    pre_root: &MerkleRoot,
    post_root: &MerkleRoot,
) -> Vec<u64> {
    vec![
        vault_id.0,
        pre_root.0,
        post_root.0,
    ]
}

/// Hashes proof components into a unique identity for tracking or caching
pub fn compute_proof_fingerprint(inputs: &[u64]) -> [u64; 3] {
    poseidon_hash(inputs)
}

/// Verifies that roots are different (i.e., valid state transition)
pub fn roots_changed(pre: &MerkleRoot, post: &MerkleRoot) -> bool {
    pre.0 != post.0
}

/// Validates a vault state transition input set
pub fn validate_zk_transition(
    vault_id: &VaultId,
    pre_root: &MerkleRoot,
    post_root: &MerkleRoot,
) -> Result<(), String> {
    if vault_id.0 == 0 {
        return Err("Vault ID cannot be zero".into());
    }

    if !roots_changed(pre, post) {
        return Err("Vault Merkle root did not change".into());
    }

    Ok(())
}
