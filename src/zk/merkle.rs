// ======================================================
// zk/merkle.rs : Merkle Tree Support for ZK Verification
// ======================================================

use serde::{Serialize, Deserialize};

/// A simple representation of a Merkle delta after a trade.
/// Used as part of ZK proof input.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleDelta {
    pub before_root: String,
    pub after_root: String,
    pub affected_leaf: String,
}

/// Verifies that a given leaf belongs to a Merkle root using its proof path.
/// NOTE: Assumes all hashes are hex-encoded strings and hashing is Poseidon.
pub fn verify_merkle_proof(
    leaf: &str,
    root: &str,
    path: &[String],
    mut index: usize,
) -> bool {
    let mut current = leaf.to_string();

    for sibling in path {
        let combined = if index % 2 == 0 {
            poseidon_hash_pair(&current, sibling)
        } else {
            poseidon_hash_pair(sibling, &current)
        };
        current = combined;
        index /= 2;
    }

    current == *root
}

/// Mock Poseidon hash for a pair of hex strings.
/// Replace this with actual Poseidon implementation in production.
fn poseidon_hash_pair(left: &str, right: &str) -> String {
    format!("H({}+{})", left, right) //  placeholder, replace with real hash
}
