// ==========================================================
// merkle_utils.rs — Domex Merkle Tree Utilities (Poseidon-based)
// ==========================================================
//
// Provides reusable Merkle hashing logic using Poseidon.
// Used for vault state roots, validator attestation trees,
// withdrawal batch proofs, and fuel verification.
//
// Shared across token, validator, and proof systems.
//

use crate::poseidon_utils::poseidon_hash_pair;
use std::vec::Vec;

/// Computes the Merkle root of a list of leaf hashes using Poseidon.
pub fn compute_merkle_root(mut leaves: Vec<[u8; 32]>) -> [u8; 32] {
    if leaves.is_empty() {
        return [0u8; 32];
    }

    while leaves.len() > 1 {
        let mut next_level = vec![];

        for i in (0..leaves.len()).step_by(2) {
            let left = leaves[i];
            let right = if i + 1 < leaves.len() {
                leaves[i + 1]
            } else {
                left // Duplicate last node if odd number of leaves
            };

            let hashed = poseidon_hash_pair(&left, &right);
            next_level.push(hashed);
        }

        leaves = next_level;
    }

    leaves[0]
}

/// Builds the Merkle proof (authentication path) for a leaf at `index`
pub fn build_merkle_proof(leaves: &[ [u8; 32] ], index: usize) -> Option<Vec<[u8; 32]>> {
    if index >= leaves.len() || leaves.is_empty() {
        return None;
    }

    let mut path = vec![];
    let mut idx = index;
    let mut current = leaves.to_vec();

    while current.len() > 1 {
        let mut next_level = vec![];

        for i in (0..current.len()).step_by(2) {
            let left = current[i];
            let right = if i + 1 < current.len() {
                current[i + 1]
            } else {
                left
            };

            if i == idx || i + 1 == idx {
                let sibling = if i == idx {
                    right
                } else {
                    left
                };
                path.push(sibling);
                idx /= 2;
            }

            let parent = poseidon_hash_pair(&left, &right);
            next_level.push(parent);
        }

        current = next_level;
    }

    Some(path)
}

/// Verifies a Merkle proof for a given leaf and root
pub fn verify_merkle_proof(
    leaf: [u8; 32],
    proof: &[ [u8; 32] ],
    mut index: usize,
    root: [u8; 32],
) -> bool {
    let mut hash = leaf;

    for sibling in proof {
        if index % 2 == 0 {
            hash = poseidon_hash_pair(&hash, sibling);
        } else {
            hash = poseidon_hash_pair(sibling, &hash);
        }
        index /= 2;
    }

    hash == root
}
