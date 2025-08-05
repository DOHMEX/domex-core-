// ========================================================
// hash_utils.rs — Domex Poseidon Hash Helpers (Plonky2)
// ========================================================
// Safe field encoding utilities for ZK inputs & hashing.
// Fully compatible with Plonky2 + Polaris + GoldilocksField.

use plonky2::field::goldilocks_field::GoldilocksField;

/// Converts a u64 into a Plonky2-compatible Goldilocks field element.
///
/// # Example
/// ```
/// let field = u64_to_goldilocks(42);
/// ```
pub fn u64_to_goldilocks(n: u64) -> GoldilocksField {
    GoldilocksField::from_canonical_u64(n)
}

/// Converts the first 8 bytes of a slice into a Goldilocks field element.
/// If input is shorter than 8 bytes, it is zero-padded (little-endian).
///
/// This is safe for hashes, addresses, pubkeys, etc.
///
/// # Example
/// ```
/// let field = bytes_to_goldilocks(&[0xaa; 32]);
/// ```
pub fn bytes_to_goldilocks(bytes: &[u8]) -> GoldilocksField {
    let mut buf = [0u8; 8];
    let len = bytes.len().min(8);
    buf[..len].copy_from_slice(&bytes[..len]);
    GoldilocksField::from_canonical_u64(u64::from_le_bytes(buf))
}
