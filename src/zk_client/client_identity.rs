// ====================================================
// client_identity.rs — Domex ZK Identity Hash (Plonky2 + Goldilocks + Poseidon)
// Computes Poseidon(sk || vault_id || zk_node_id)
// ====================================================

use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::hash::poseidon::{PoseidonHash, poseidon_hash};

/// Converts a 32-byte array to a GoldilocksField element by truncation.
/// NOTE: GoldilocksField is only 64 bits, so we map from lower 8 bytes.
pub fn bytes32_to_field(input: &[u8; 32]) -> GoldilocksField {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&input[..8]); // Truncate to 64-bit
    GoldilocksField::from_canonical_u64(u64::from_le_bytes(bytes))
}

/// Converts a u64 to a GoldilocksField element
pub fn u64_to_field(value: u64) -> GoldilocksField {
    GoldilocksField::from_canonical_u64(value)
}

/// Computes Poseidon(sk || vault_id || zk_node_id) using GoldilocksField
pub fn compute_identity_hash(
    sk_bytes: &[u8; 32],
    vault_id: u64,
    zk_node_id_bytes: &[u8; 32],
) -> GoldilocksField {
    let sk_field = bytes32_to_field(sk_bytes);
    let vault_field = u64_to_field(vault_id);
    let node_field = bytes32_to_field(zk_node_id_bytes);

    poseidon_hash([sk_field, vault_field, node_field])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_hash_computation() {
        let dummy_sk = [0x11u8; 32];
        let dummy_vault_id = 777;
        let dummy_node_id = [0x22u8; 32];

        let identity_hash = compute_identity_hash(&dummy_sk, dummy_vault_id, &dummy_node_id);
        println!("Computed Identity Hash: {:?}", identity_hash.0);
    }
}
