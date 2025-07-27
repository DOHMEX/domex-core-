// ===============================================================
// zk/merkle.rs : Merkle Tree Verification using Poseidon Hash
// ============================================================

use plonky2_field::goldilocks_field::GoldilocksField;
use plonky2_hash::poseidon::{PoseidonHash, poseidon_hash};
use serde::{Deserialize, Serialize};

/// Merkle root delta (used in ZK circuit)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleDelta {
    pub before_root: String,
    pub after_root: String,
    pub affected_leaf: String,
}

/// Verifies that a given leaf belongs to a Merkle root using a Poseidon-based proof path.
/// 
/// Inputs are expected as hex strings; they are converted into field elements internally.
/// Assumes binary Merkle tree (left/right based on index parity).
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

    current == root
}

/// Uses Plonky2 Poseidon to hash two hex strings.
fn poseidon_hash_pair(left: &str, right: &str) -> String {
    let left_field = string_to_field(left);
    let right_field = string_to_field(right);

    let result: [GoldilocksField; 1] = poseidon_hash::<GoldilocksField, 2>([left_field, right_field]);
    field_to_hex(&result[0])
}

/// Converts hex or string to Goldilocks field element.
fn string_to_field(input: &str) -> GoldilocksField {
    // Convert from hex string → u64 → field
    let cleaned = input.trim_start_matches("0x");
    let val = u64::from_str_radix(cleaned, 16).unwrap_or(0);
    GoldilocksField::from_canonical_u64(val)
}

/// Converts field element back to hex string.
fn field_to_hex(field: &GoldilocksField) -> String {
    format!("{:x}", field.0)
}
