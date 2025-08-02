// ====================================================
// client_identity.rs — Domex ZK Identity Hash (Ponkey2 + Pasta)
// Computes Poseidon(sk || vault_id || zk_node_id)
// ====================================================

use ponkey2_poseidon::PoseidonHasher;
use pasta_curves::Fp;

/// Converts a 32-byte array to a Pasta field element
pub fn bytes_to_fp(input: &[u8; 32]) -> Fp {
    Fp::from_bytes(input).expect("Invalid Pasta field element")
}

/// Converts a u64 vault ID to a Pasta field element
pub fn u64_to_fp(value: u64) -> Fp {
    Fp::from(value)
}

/// Computes Poseidon(sk || vault_id || zk_node_id) as Fp field element
pub fn compute_identity_hash(
    sk_bytes: &[u8; 32],
    vault_id: u64,
    zk_node_id_bytes: &[u8; 32],
) -> Fp {
    let sk_fp = bytes_to_fp(sk_bytes);
    let vault_fp = u64_to_fp(vault_id);
    let node_fp = bytes_to_fp(zk_node_id_bytes);

    let mut hasher = PoseidonHasher::new();
    hasher.hash(&[sk_fp, vault_fp, node_fp])
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
        println!("Computed Identity Hash: {:?}", identity_hash.to_bytes());
    }
}
