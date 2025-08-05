// src/zk_client/deposit_address.rs

use crate::crypto::poseidon_hasher::PoseidonHasher;
use crate::types::common::{Token, ZkIdentity, DepositAddress};

/// Error type for deposit address generation
#[derive(Debug)]
pub enum AddressGenerationError {
    InvalidIdentityLength,
    EmptyToken,
}

/// Generates a deterministic deposit address based on a user's Poseidon identity and token type.
///
/// Formula: `hash(Poseidon(pubkey) || token_type)`
///
/// Returns a Result with the deposit address, or an error if inputs are invalid.
pub fn generate_deposit_address(
    poseidon_identity: &ZkIdentity,
    token_type: &Token,
) -> Result<DepositAddress, AddressGenerationError> {
    let id_bytes = poseidon_identity.as_bytes();
    let token_bytes = token_type.as_bytes();

    // Input validation
    if id_bytes.len() != 32 {
        return Err(AddressGenerationError::InvalidIdentityLength);
    }
    if token_bytes.is_empty() {
        return Err(AddressGenerationError::EmptyToken);
    }

    let combined = [id_bytes, token_bytes].concat(); // Poseidon(pubkey) || token_type
    let hashed = PoseidonHasher::hash(&combined);    // 32-byte hash
    Ok(DepositAddress::from_hash(hashed))
}
