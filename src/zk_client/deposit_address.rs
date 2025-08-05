// ==========================================================
// deposit_address.rs — Domex Deterministic Deposit Address (w/ Delegation)
// ==========================================================

use crate::crypto::poseidon_hasher::PoseidonHasher;
use crate::types::common::{Token, ZkIdentity, DepositAddress};

/// Error type for deposit address generation
#[derive(Debug)]
pub enum AddressGenerationError {
    InvalidIdentityLength,
    EmptyToken,
    InvalidDelegatePubkeyLength,
}

/// Generates a deposit address based on:
/// Poseidon(pubkey) || token_type || delegate_pubkey
///
/// This ensures that only a specific delegate is authorized to manage the deposit.
///
/// # Arguments:
/// - `poseidon_identity`: 32-byte identity hash (Poseidon(pubkey))
/// - `token_type`: Token ticker (e.g., "BTC")
/// - `delegate_pubkey`: 32-byte public key of the delegate wallet
///
/// # Returns:
/// - Deterministic deposit address unique to this (identity, token, delegate) combination
pub fn generate_deposit_address_with_delegate(
    poseidon_identity: &ZkIdentity,
    token_type: &Token,
    delegate_pubkey: &[u8; 32],
) -> Result<DepositAddress, AddressGenerationError> {
    let id_bytes = poseidon_identity.as_bytes();
    let token_bytes = token_type.as_bytes();

    // === Input Validation ===
    if id_bytes.len() != 32 {
        return Err(AddressGenerationError::InvalidIdentityLength);
    }
    if token_bytes.is_empty() {
        return Err(AddressGenerationError::EmptyToken);
    }

    // === Combine Poseidon(pubkey) || token_type || delegate_pubkey ===
    let combined = [id_bytes, token_bytes, delegate_pubkey].concat();
    let hashed = PoseidonHasher::hash(&combined);

    Ok(DepositAddress::from_hash(hashed))
}
