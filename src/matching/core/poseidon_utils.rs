// ===============================
// poseidon_utils.rs : Domex Poseidon Hash Utilities (Plonky2 + Pasta Fp)
// ===============================

use pasta_curves::Fp;
use plonky2_poseidon::PoseidonHasher;

/// Converts a 32-byte input to Pasta Fp (used as base field in Plonky2 circuits)
pub fn bytes_to_fp(input: &[u8; 32]) -> Fp {
    Fp::from_bytes(input).expect("Invalid bytes: not a valid Pasta field element")
}

/// Converts a u64 (e.g., vault ID or token amount) to Pasta Fp
pub fn u64_to_fp(value: u64) -> Fp {
    Fp::from(value)
}

/// Converts a UTF-8 string (e.g., pubkey or vault_id) to Pasta Fp
pub fn string_to_fp(input: &str) -> Fp {
    let mut buf = [0u8; 32];
    let bytes = input.as_bytes();
    let len = bytes.len().min(32);
    buf[..len].copy_from_slice(&bytes[..len]);
    Fp::from_bytes(&buf).expect("Invalid Pasta field element from string")
}

/// Computes Poseidon(sk || vault_id || zk_node_id) — identity hash for onboarding
pub fn recompute_identity_hash_from_fp(sk_fp: Fp, vault_fp: Fp, node_fp: Fp) -> Fp {
    let mut hasher = PoseidonHasher::new();
    hasher.hash(&[sk_fp, vault_fp, node_fp])
}

/// Computes Poseidon(vault_id || delegate_pubkey) — used for delegation binding
pub fn recompute_delegation_hash(vault_id: &str, delegate_pubkey: &str) -> String {
    let fp1 = string_to_fp(vault_id);
    let fp2 = string_to_fp(delegate_pubkey);
    let mut hasher = PoseidonHasher::new();
    let hash = hasher.hash(&[fp1, fp2]);
    hex::encode(hash.to_bytes())
}

/// Verifies that identity hash matches Poseidon(sk || vault_id || zk_node_id)
pub fn verify_identity_hash(expected_hash: Fp, sk_fp: Fp, vault_id: u64, zk_node_id: [u8; 32]) -> bool {
    let vault_fp = u64_to_fp(vault_id);
    let node_fp = bytes_to_fp(&zk_node_id);
    let recomputed = recompute_identity_hash_from_fp(sk_fp, vault_fp, node_fp);
    expected_hash == recomputed
}

/// Computes Poseidon(sk || lock_script_hash || withdraw_amount) — withdrawal lock hash
pub fn recompute_lock_withdraw_hash(sk_fp: Fp, script_hash_fp: Fp, amount_fp: Fp) -> Fp {
    let mut hasher = PoseidonHasher::new();
    hasher.hash(&[sk_fp, script_hash_fp, amount_fp])
}

/// Verifies withdrawal lock hash matches recomputed version
pub fn verify_lock_withdraw_hash(expected_hash: Fp, sk_fp: Fp, script_hash: [u8; 32], withdraw_amount: u64) -> bool {
    let script_fp = bytes_to_fp(&script_hash);
    let amount_fp = u64_to_fp(withdraw_amount);
    let recomputed = recompute_lock_withdraw_hash(sk_fp, script_fp, amount_fp);
    expected_hash == recomputed
}
