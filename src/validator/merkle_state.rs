// src/validator/merkle_state.rs

// Domex Merkle State Handler
// Builds and verifies Merkle roots over vault balances

use crate::types::merkle::{MerkleLeaf, MerkleRoot, MerkleProof};
use crate::types::vault::VaultState;
use crate::utils::poseidon::poseidon_hash_pair;
use std::collections::BTreeMap;

/// Computes the Merkle root from a VaultState
/// Each leaf = Poseidon(identity + token + balance)
pub fn compute_merkle_root(state: &VaultState) -> (MerkleRoot, Vec<MerkleLeaf>) {
    let mut leaves: Vec<MerkleLeaf> = vec![];

    for ((identity, token), balance) in state.balances.iter() {
        let leaf = MerkleLeaf {
            key: format!("{}::{}", identity, token),
            value: balance.to_string(),
        };
        leaves.push(leaf);
    }

    // Sort leaves lexicographically to ensure deterministic Merkle tree
    leaves.sort_by(|a, b| a.key.cmp(&b.key));

    let mut hashed_leaves: Vec<String> = leaves
        .iter()
        .map(|leaf| poseidon_hash_pair(&leaf.key, &leaf.value))
        .collect();

    while hashed_leaves.len() > 1 {
        let mut next_layer = vec![];
        let mut iter = hashed_leaves.chunks(2);

        for pair in iter {
            let combined = if pair.len() == 2 {
                poseidon_hash_pair(&pair[0], &pair[1])
            } else {
                pair[0].clone()
            };
            next_layer.push(combined);
        }

        hashed_leaves = next_layer;
    }

    let root = hashed_leaves.first().cloned().unwrap_or_else(|| "0".repeat(64));

    (MerkleRoot(root), leaves)
}

/// Verifies a Merkle proof against a known Merkle root
pub fn verify_merkle_proof(
    root: &MerkleRoot,
    leaf: &MerkleLeaf,
    proof: &MerkleProof,
) -> bool {
    let mut hash = poseidon_hash_pair(&leaf.key, &leaf.value);

    for (sibling_hash, is_right) in &proof.path {
        hash = if *is_right {
            poseidon_hash_pair(sibling_hash, &hash)
        } else {
            poseidon_hash_pair(&hash, sibling_hash)
        };
    }

    &hash == &root.0
}
