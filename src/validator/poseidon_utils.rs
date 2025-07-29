// poseidon_utils.rs
// Domex validator-side Poseidon hash utilities (Ponkey2 + Pasta)
// Used to recompute zk onboarding identity hashes

use pasta_curves::Fp;
use ponkey2_poseidon::PoseidonHasher;

/// Converts a 32-byte field-safe input to Pasta Fp
pub fn bytes_to_fp(input: &[u8; 32]) -> Fp {
    Fp::from_bytes(input).expect("Invalid bytes: not a valid Pasta field element")
}

/// Converts a u64 (e.g., vault ID) to Pasta Fp
pub fn u64_to_fp(value: u64) -> Fp {
    Fp::from(value)
}

/// Validator-side Poseidon recomputation:
/// Reconstructs Poseidon(sk || vault_id || zk_node_id) from internal inputs
/// sk_fp is passed only if the circuit leaks its hash for recomputation purposes
pub fn recompute_identity_hash_from_fp(
    sk_fp: Fp,
    vault_fp: Fp,
    node_fp: Fp,
) -> Fp {
    let mut hasher = PoseidonHasher::new();
    hasher.hash(&[sk_fp, vault_fp, node_fp])
}

/// Optional: Verifies that identity hash matches recomputed Poseidon
pub fn verify_identity_hash(
    expected_hash: Fp,
    sk_fp: Fp,
    vault_id: u64,
    zk_node_id: [u8; 32],
) -> bool {
    let vault_fp = u64_to_fp(vault_id);
    let node_fp = bytes_to_fp(&zk_node_id);
    let recomputed = recompute_identity_hash_from_fp(sk_fp, vault_fp, node_fp);
    expected_hash == recomputed
}
