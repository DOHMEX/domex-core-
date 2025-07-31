// poseidon_utils.rs
// Domex validator-side Poseidon hash utilities (Ponkey2 + Pasta)
// Used to recompute zk onboarding identity hashes and verify lock-script bindings

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

/// Reconstructs Poseidon(sk || vault_id || zk_node_id)
pub fn recompute_identity_hash_from_fp(
    sk_fp: Fp,
    vault_fp: Fp,
    node_fp: Fp,
) -> Fp {
    let mut hasher = PoseidonHasher::new();
    hasher.hash(&[sk_fp, vault_fp, node_fp])
}

/// Optional: Recompute lock-script hash for onchain_guarded mode
/// Poseidon(sk || lock_script_hash || withdraw_amount)
pub fn recompute_lock_withdraw_hash(
    sk_fp: Fp,
    script_hash_fp: Fp,
    amount_fp: Fp,
) -> Fp {
    let mut hasher = PoseidonHasher::new();
    hasher.hash(&[sk_fp, script_hash_fp, amount_fp])
}

/// Verifies a standard identity hash from onboarding intent
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

/// Verifies a withdrawal-mode script hash binding (off-chain wallet lock)
pub fn verify_lock_withdraw_hash(
    expected_hash: Fp,
    sk_fp: Fp,
    script_hash: [u8; 32],
    withdraw_amount: u64,
) -> bool {
    let script_fp = bytes_to_fp(&script_hash);
    let amount_fp = u64_to_fp(withdraw_amount);
    let recomputed = recompute_lock_withdraw_hash(sk_fp, script_fp, amount_fp);
    expected_hash == recomputed
}
