// hash_utils.rs
// Utilities for hashing and field encoding in Domex zk onboarding
// Uses Ponkey2 Poseidon and Pasta curves

use pasta_curves::Fp;
use ponkey2_poseidon::PoseidonHasher;

/// Converts a 32-byte array to a Pasta Fp field element.
/// Panics if the input is not a valid field element.
pub fn bytes_to_fp(input: &[u8; 32]) -> Fp {
    Fp::from_bytes(input).expect("Invalid bytes: not a valid Pasta field element")
}

/// Converts a u64 to Fp directly
pub fn u64_to_fp(value: u64) -> Fp {
    Fp::from(value)
}

/// Computes Poseidon hash of a list of Fp inputs.
/// Supports 2–5 inputs, rejects empty or oversized input arrays.
pub fn poseidon_hash(inputs: &[Fp]) -> Fp {
    assert!(!inputs.is_empty(), "Poseidon input list cannot be empty");
    assert!(inputs.len() <= 5, "Too many inputs for this Poseidon instance");

    let mut hasher = PoseidonHasher::new();
    hasher.hash(inputs)
}
