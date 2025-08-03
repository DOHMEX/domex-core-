// ==============================================
// deposit_address.rs — Domex Identity Key Deriver (Plonky2 Compatible)
// ==============================================
// Derives a deterministic public identity field from a 32-byte secret key
// using Poseidon hash over GoldilocksField. Used in ZK onboarding.

use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2_poseidon::PoseidonHash;
use plonky2_poseidon::poseidon_hash;
use plonky2::field::types::Field;

/// Derives a public key hash from a 32-byte secret key using Poseidon over Goldilocks.
/// This replaces elliptic curve pubkey (x, y) in Domex under Plonky2.
pub fn derive_identity_fp(sk_bytes: &[u8; 32]) -> GoldilocksField {
    // Take first 8 bytes of sk and convert to GoldilocksField
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&sk_bytes[..8]);
    let sk_u64 = u64::from_le_bytes(buf);
    let sk_field = GoldilocksField::from_canonical_u64(sk_u64);

    // Hash the field to get identity point
    poseidon_hash([sk_field])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_hash_derivation() {
        let sk = [1u8; 32]; // Dummy secret key
        let identity = derive_identity_fp(&sk);
        println!("Derived Identity Field (Poseidon over Goldilocks): {:?}", identity);
    }
}
