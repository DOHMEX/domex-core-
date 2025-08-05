// ===============================================================
// delegate_utils.rs — Domex Delegate & Delegator Hash Logic (Plonky2 + Poseidon)
// ===============================================================
//
// Supports vault-specific delegation of ZK proof rights (claim + withdrawal),
// and identification of fuel-bearing executors (delegators).

use crate::poseidon_utils::{u64_to_goldilocks, bytes_to_goldilocks};
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::hash::poseidon::poseidon_hash;

/// Computes the delegate authorization hash:
///     Poseidon(vault_id || delegate_pubkey)
///
/// This binds a delegate wallet to a specific vault. Used to authorize
/// deposit claims or withdrawals by a 3rd party.
///
/// # Arguments:
/// - `vault_id`: u64 vault ID (e.g., BTC/USDT → 777)
/// - `delegate_pubkey`: 32-byte public key of the delegate wallet
///
/// # Returns:
/// - GoldilocksField representing the delegation hash
pub fn compute_delegate_hash(
    vault_id: u64,
    delegate_pubkey: &[u8; 32],
) -> GoldilocksField {
    let vault_fp = u64_to_goldilocks(vault_id);
    let pubkey_fp = bytes_to_goldilocks(delegate_pubkey);

    poseidon_hash([vault_fp, pubkey_fp])
}

/// Verifies that a delegate hash matches the claimed vault + pubkey
pub fn verify_delegate_hash(
    expected_hash: &GoldilocksField,
    vault_id: u64,
    delegate_pubkey: &[u8; 32],
) -> bool {
    let computed = compute_delegate_hash(vault_id, delegate_pubkey);
    computed == *expected_hash
}

/// (Optional) Computes a "delegator identity" — useful for tracking who burned fuel
///
/// This does not grant rights — it’s just a readable hash of the fuel-submitting party
pub fn compute_delegator_identity(delegate_pubkey: &[u8; 32]) -> GoldilocksField {
    bytes_to_goldilocks(delegate_pubkey)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delegate_hash_consistency() {
        let vault_id = 42;
        let delegate_pubkey = [0x11u8; 32];

        let h1 = compute_delegate_hash(vault_id, &delegate_pubkey);
        let h2 = compute_delegate_hash(vault_id, &delegate_pubkey);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_delegate_verification() {
        let vault_id = 1234;
        let pubkey = [0x22u8; 32];
        let expected = compute_delegate_hash(vault_id, &pubkey);
        assert!(verify_delegate_hash(&expected, vault_id, &pubkey));
    }

    #[test]
    fn test_delegator_identity_deterministic() {
        let pubkey = [0xABu8; 32];
        let id1 = compute_delegator_identity(&pubkey);
        let id2 = compute_delegator_identity(&pubkey);
        assert_eq!(id1, id2);
    }
}
