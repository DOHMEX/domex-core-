// ===============================
// identity.rs — Domex Poseidon Identity (Plonky2 + Pasta Curves)
// ===============================

use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::hash::poseidon::{PoseidonHash, PoseidonPermutation};
use plonky2::iop::witness::PartialWitness;

use crate::types::{VaultId, DelegationHash};

/// Plonky2 field used across Domex (can switch to Pasta if circuit changes)
type F = GoldilocksField;

/// Verifies that the user-submitted Poseidon hash matches the registered vault owner.
pub fn verify_poseidon_auth(submitted_hash: &str, vault_id: &VaultId) -> bool {
    crate::vault_registry::get_owner_for_vault(vault_id)
        .map_or(false, |registered| registered == *submitted_hash)
}

/// Computes Poseidon(vault_id + delegate_pubkey) for delegation
pub fn compute_delegation_hash(vault_id: &VaultId, delegate_pubkey: &str) -> DelegationHash {
    let input1 = string_to_field(vault_id);
    let input2 = string_to_field(delegate_pubkey);

    let mut hasher = PoseidonPermutation::<F>::default();
    let hash = hasher.hash_or_noop(&[input1, input2]);

    hex::encode(hash.to_canonical_u64().to_le_bytes())
}

/// Converts a string (e.g., pubkey) to a field element compatible with Plonky2
fn string_to_field(input: &str) -> F {
    let bytes = input.as_bytes();
    let mut buf = [0u8; 8]; // u64-sized
    for (i, byte) in bytes.iter().take(8).enumerate() {
        buf[i] = *byte;
    }
    F::from_canonical_u64(u64::from_le_bytes(buf))
}
