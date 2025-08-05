// ==========================================================
// deposit_address.rs — Domex Deterministic Deposit Address
// ==========================================================
// Generates vault-specific, identity-bound deposit addresses.
// Fully deterministic: hash(Poseidon(pubkey) || token_type)

use crate::crypto::poseidon_hasher::PoseidonHasher;
use crate::types::common::{Token, ZkIdentity, DepositAddress};

/// Error type for deposit address generation
#[derive(Debug)]
pub enum AddressGenerationError {
    InvalidIdentityLength,
    EmptyToken,
}

/// Generates a deterministic deposit address from a user's Poseidon identity + token.
///
/// Formula:
///     hash(Poseidon(pubkey) || token_type)
///
/// This ensures:
/// - Same identity + token = same address
/// - No collisions across identities or tokens
///
/// # Arguments:
/// - `poseidon_identity`: 32-byte identity hash (already Poseidon(pubkey))
/// - `token_type`: Token ticker string (e.g. "BTC", "ETH")
///
/// # Returns:
/// - `Ok(DepositAddress)` if valid
/// - `Err(AddressGenerationError)` if invalid
pub fn generate_deposit_address(
    poseidon_identity: &ZkIdentity,
    token_type: &Token,
) -> Result<DepositAddress, AddressGenerationError> {
    let id_bytes = poseidon_identity.as_bytes();
    let token_bytes = token_type.as_bytes();

    // === Validation ===
    if id_bytes.len() != 32 {
        return Err(AddressGenerationError::InvalidIdentityLength);
    }
    if token_bytes.is_empty() {
        return Err(AddressGenerationError::EmptyToken);
    }

    // === Deterministic Construction ===
    let combined = [id_bytes, token_bytes].concat();  // Poseidon(pubkey) || token_type
    let hashed = PoseidonHasher::hash(&combined);     // 32-byte hash result

    Ok(DepositAddress::from_hash(hashed))             // Maps to BTC/SOL/ETH-format address
}
