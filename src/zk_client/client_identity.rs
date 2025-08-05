// ====================================================
// client_identity.rs — Domex ZK Identity Hash (Plonky2 + Goldilocks + Poseidon)
// Computes Poseidon(sk || vault_id || zk_node_id)
// ====================================================

use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::hash::poseidon::poseidon_hash;

/// Converts the first 8 bytes of a 32-byte input to a Goldilocks field element.
/// Used to safely embed a secret key or node ID into ZK circuits.
pub fn bytes32_to_field(input: &[u8; 32]) -> GoldilocksField {
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&input[..8]); // Take least-significant 64 bits
    GoldilocksField::from_canonical_u64(u64::from_le_bytes(buf))
}

/// Converts a u64 (e.g., vault ID) into a Goldilocks field element.
pub fn u64_to_field(value: u64) -> GoldilocksField {
    GoldilocksField::from_canonical_u64(value)
}

/// Computes the Poseidon identity hash: Poseidon(sk || vault_id || zk_node_id)
///
/// This forms the ZK user identity in Domex and is used for:
/// - Vault ownership
/// - Deposit address derivation
/// - Withdrawal validation
pub fn compute_poseidon_identity(
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
    fn test_poseidon_identity_hash() {
        let dummy_sk = [0x11u8; 32];
        let dummy_vault_id = 777;
        let dummy_node_id = [0x22u8; 32];

        let identity_hash = compute_poseidon_identity(&dummy_sk, dummy_vault_id, &dummy_node_id);
        println!("Computed Identity Hash: {:?}", identity_hash.0);
    }
}
