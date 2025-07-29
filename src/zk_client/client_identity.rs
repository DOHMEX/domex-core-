// client_identity.rs
// Domex zk onboarding identity module
// Computes Poseidon(sk || vault_id || zk_node_id)

use ponkey2_poseidon::PoseidonHasher;
use pasta_curves::Fp;
use std::convert::TryInto;

/// Converts a 32-byte array to a Pasta field element
fn to_fp(input: &[u8; 32]) -> Fp {
    Fp::from_bytes(input).expect("Invalid field element")
}

/// Converts a u64 vault ID to a Pasta field element (you can change this to use hash or u256)
fn vault_id_to_fp(vault_id: u64) -> Fp {
    Fp::from(vault_id)
}

/// Converts a zk-node ID to a Pasta field element (e.g., node ID hash)
fn zk_node_id_to_fp(zk_node_id: &[u8; 32]) -> Fp {
    to_fp(zk_node_id)
}

/// Computes Poseidon(sk || vault_id || zk_node_id)
pub fn compute_identity_hash(
    sk_bytes: &[u8; 32],
    vault_id: u64,
    zk_node_id_bytes: &[u8; 32],
) -> Fp {
    let sk_fp = to_fp(sk_bytes);
    let vault_fp = vault_id_to_fp(vault_id);
    let node_fp = zk_node_id_to_fp(zk_node_id_bytes);

    let mut hasher = PoseidonHasher::new();

    hasher.hash(&[sk_fp, vault_fp, node_fp])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poseidon_identity_hash() {
        let sk = [1u8; 32]; // Dummy private key
        let vault_id = 42;
        let node_id = [2u8; 32];

        let identity = compute_identity_hash(&sk, vault_id, &node_id);
        println!("Identity Hash (Fp): {:?}", identity);
    }
}
