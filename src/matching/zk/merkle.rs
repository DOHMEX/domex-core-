// ===============================================================
// zk/merkle.rs — Domex Merkle Tree Verification using Ponkey2 + Pasta Poseidon
// ===============================================================

use pasta_curves::Fp;
use ponkey2_poseidon::PoseidonHasher;
use serde::{Deserialize, Serialize};

/// Merkle root delta (used in ZK circuit context)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleDelta {
    pub before_root: String,
    pub after_root: String,
    pub affected_leaf: String,
}

/// Verifies that a given leaf belongs to a Merkle root using a Poseidon-based Merkle proof.
/// All inputs are hex strings encoded from Pasta Fp field elements.
pub fn verify_merkle_proof(
    leaf_hex: &str,
    root_hex: &str,
    path: &[String],
    mut index: usize,
) -> bool {
    let mut current = hex_to_fp(leaf_hex);

    for sibling_hex in path {
        let sibling = hex_to_fp(sibling_hex);
        let combined = if index % 2 == 0 {
            hash_pair(current, sibling)
        } else {
            hash_pair(sibling, current)
        };
        current = combined;
        index /= 2;
    }

    current == hex_to_fp(root_hex)
}

/// Hashes two Pasta Fp values using Poseidon (Ponkey2-compatible)
fn hash_pair(left: Fp, right: Fp) -> Fp {
    let mut hasher = PoseidonHasher::new();
    hasher.hash(&[left, right])
}

/// Converts a hex string into a Pasta field element (Fp)
fn hex_to_fp(hex_str: &str) -> Fp {
    let cleaned = hex_str.trim_start_matches("0x");
    let mut bytes = [0u8; 32];
    hex::decode_to_slice(cleaned, &mut bytes[..]).expect("Invalid hex input");
    Fp::from_bytes(&bytes).expect("Invalid Pasta field element")
}

/// Converts Fp field element into hex string
pub fn fp_to_hex(fp: &Fp) -> String {
    hex::encode(fp.to_bytes())
}
