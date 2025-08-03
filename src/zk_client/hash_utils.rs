// ============================================
// hash_utils.rs — Domex Poseidon Hash Helpers
// ============================================
// Hashing and field encoding utilities using Ponkey2 + Pasta curves.
// Supports identity, delegation, and withdrawal hashing.

use plonky2::field::goldilocks_field::GoldilocksField;

/// Converts u64 to GoldilocksField
pub fn u64_to_goldilocks(n: u64) -> GoldilocksField {
    GoldilocksField::from_canonical_u64(n)
}

/// Converts 32-byte slice to GoldilocksField (first 8 bytes little endian)
pub fn bytes_to_goldilocks(bytes: &[u8]) -> GoldilocksField {
    let truncated = &bytes[..8.min(bytes.len())];
    let array = truncated.try_into().unwrap_or([0u8; 8]);
    GoldilocksField::from_canonical_u64(u64::from_le_bytes(array))
}
