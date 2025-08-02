// ============================================
// hash_utils.rs — Domex Poseidon Hash Helpers
// ============================================
// Hashing and field encoding utilities using Ponkey2 + Pasta curves.
// Supports identity, delegation, and withdrawal hashing.

use pasta_curves::Fp;
use ponkey2_poseidon::PoseidonHasher;

/// Converts a 32-byte array to a Pasta Fp field element.
/// Panics if input is invalid (non-canonical).
pub fn bytes_to_fp(input: &[u8; 32]) -> Fp {
    Fp::from_bytes(input).expect("Invalid bytes: not a valid Pasta field element")
}

/// Converts a u64 (e.g., vault ID or amount) to Pasta Fp.
pub fn u64_to_fp(value: u64) -> Fp {
    Fp::from(value)
}

/// Computes Poseidon hash over a list of Pasta field elements.
///
/// # Panics
/// Panics if input list is empty or exceeds 5 elements.
pub fn poseidon_hash(inputs: &[Fp]) -> Fp {
    assert!(!inputs.is_empty(), "Poseidon input list cannot be empty");
    assert!(inputs.len() <= 5, "Too many inputs for this Poseidon instance");

    let mut hasher = PoseidonHasher::new();
    hasher.hash(inputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poseidon_hash_with_two_inputs() {
        let a = u64_to_fp(10);
        let b = u64_to_fp(42);
        let hash = poseidon_hash(&[a, b]);
        println!("Hash(10, 42) = {:?}", hash);
    }
}
