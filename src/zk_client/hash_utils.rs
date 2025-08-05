// ============================================
// hash_utils.rs — Domex Poseidon Hash Helpers
// ============================================
// Hashing and field encoding utilities using Plonky2.
// Supports identity, delegation, vault proofs, and withdrawal hashing.

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
///
/// - If the slice is shorter than 8 bytes, it is zero-padded.
/// - If longer, only the first 8 bytes are used (little-endian).
///
/// # Example
/// ```
/// let hash = [1u8; 32]; // or tx_hash, identity hash, etc.
/// let field = bytes_to_goldilocks(&hash);
/// ```
pub fn bytes_to_goldilocks(bytes: &[u8]) -> GoldilocksField {
    let mut buf = [0u8; 8];
    buf[..bytes.len().min(8)].copy_from_slice(&bytes[..bytes.len().min(8)]);
    GoldilocksField::from_canonical_u64(u64::from_le_bytes(buf))
}
