// ==========================================================
// identity_utils.rs — Identity Hashing & Poseidon Utilities
// ==========================================================
//
// Utilities for generating and verifying Domex identities
// via Poseidon hashing. Used for vault ownership, delegation,
// validator ID derivation, and slashing logs.
//

use poseidon_rs::Poseidon;
use crate::common_types::VaultId;

/// Generates a Poseidon-based identity hash from public key
///
/// # Arguments
/// - `pubkey`: 32-byte raw public key
///
/// # Returns
/// - 32-byte Poseidon identity hash
pub fn poseidon_identity_hash(pubkey: &[u8; 32]) -> [u8; 32] {
    let mut poseidon = Poseidon::new(2); // 2 input elements
    poseidon.update(&[bytes_to_field(pubkey), 0.into()]);
    field_to_bytes(&poseidon.squeeze())
}

/// Generates a delegation hash: Poseidon(vault_id || pubkey || nonce)
///
/// # Arguments
/// - `vault_id`: Vault identifier
/// - `pubkey`: Delegator public key
/// - `nonce`: Unique delegation session number
///
/// # Returns
/// - 32-byte Poseidon delegation hash
pub fn generate_delegation_hash(vault_id: &VaultId, pubkey: &[u8; 32], nonce: u64) -> [u8; 32] {
    let mut poseidon = Poseidon::new(3);
    poseidon.update(&[
        bytes_to_field(vault_id),
        bytes_to_field(pubkey),
        (nonce as u128).into(),
    ]);
    field_to_bytes(&poseidon.squeeze())
}

/// Converts 32-byte array into Poseidon field element
fn bytes_to_field(input: &[u8; 32]) -> poseidon_rs::Field {
    use poseidon_rs::Field;
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(input);
    Field::from_bytes_wide(&bytes)
}

/// Converts Poseidon field element to 32-byte array
fn field_to_bytes(field: &poseidon_rs::Field) -> [u8; 32] {
    let mut out = [0u8; 32];
    out.copy_from_slice(&field.to_bytes());
    out
}
