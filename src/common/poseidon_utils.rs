// =============================================================
// poseidon_utils.rs — Domex Poseidon Hashing Utilities
// =============================================================
//
// Provides Poseidon-based hashing utilities with domain separation
// for consistent and secure hash generation across Domex.
// Used in validator attestations, delegation proofs, fuel tracking,
// vault identity registration, and Merkle tree operations.
//
// Note: Uses poseidon_rs (https://github.com/arnaucube/poseidon_rs)
//
// =============================================================

use poseidon_rs::{Poseidon, PoseidonConstants};

/// Hashes a fixed array of field elements with Poseidon and domain separation.
///
/// # Arguments
/// * `inputs` - A slice of u64 values representing field elements.
/// * `domain_tag` - A unique domain separator to prevent cross-context collisions.
///
/// # Returns
/// A single 32-byte Poseidon hash digest.
pub fn poseidon_hash(inputs: &[u64], domain_tag: u64) -> [u8; 32] {
    let mut poseidon = Poseidon::new();
    poseidon.init();

    // Inject domain tag as prefix
    poseidon.update(&[domain_tag]);
    poseidon.update(inputs);

    let result = poseidon.squeeze();
    result.to_be_bytes()
}

/// Hashes two u64 values with Poseidon using a domain separator.
///
/// # Example: Poseidon(vault_id, nonce) with domain separation
pub fn poseidon_hash2(a: u64, b: u64, domain_tag: u64) -> [u8; 32] {
    poseidon_hash(&[a, b], domain_tag)
}

/// Hashes three u64 values with Poseidon using domain separation.
pub fn poseidon_hash3(a: u64, b: u64, c: u64, domain_tag: u64) -> [u8; 32] {
    poseidon_hash(&[a, b, c], domain_tag)
}

/// Poseidon-based Merkle leaf construction.
/// Used in vault trees and validator attestation Merkle trees.
pub fn poseidon_leaf_hash(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let left_u64 = u64::from_be_bytes(left[0..8].try_into().unwrap());
    let right_u64 = u64::from_be_bytes(right[0..8].try_into().unwrap());
    poseidon_hash(&[left_u64, right_u64], 0xA1) // 0xA1 = domain for Merkle leaf
}

/// Unique vault identity hash from vault owner + token + timestamp
pub fn vault_identity_hash(owner: u64, token_id: u64, timestamp: u64) -> [u8; 32] {
    poseidon_hash3(owner, token_id, timestamp, 0xB2) // 0xB2 = vault ID context
}

/// Delegation hash: Poseidon(vault_id, delegate_pubkey, nonce)
pub fn delegation_hash(vault_id: u64, delegate_pubkey: u64, nonce: u64) -> [u8; 32] {
    poseidon_hash3(vault_id, delegate_pubkey, nonce, 0xC7) // 0xC7 = delegation domain
}

/// Fuel burn hash: Poseidon(vault_id, amount, nonce)
pub fn fuel_burn_hash(vault_id: u64, amount: u64, nonce: u64) -> [u8; 32] {
    poseidon_hash3(vault_id, amount, nonce, 0xD9) // 0xD9 = fuel domain
}
