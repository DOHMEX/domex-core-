// src/zk_client/deposit_address.rs

use crate::crypto::poseidon_hasher::PoseidonHasher;
use crate::types::common::{Token, ZkIdentity, DepositAddress};

/// Generates a deterministic deposit address based on a user's Poseidon identity and token type.
///
/// Formula: `hash(Poseidon(pubkey) || token_type)`
///
/// This address is used by the deposit watcher script and is guaranteed to be unique
/// per (user, token) pair. The same identity and token will always produce the same address.
///
/// # Arguments
/// - `poseidon_identity`: A ZK identity already hashed with Poseidon(pubkey)
/// - `token_type`: The token being deposited (e.g., BTC, ETH, USDT)
///
/// # Returns
/// - A deterministic `DepositAddress` usable on the target native chain (e.g., BTC address)
pub fn generate_deposit_address(
    poseidon_identity: &ZkIdentity,
    token_type: &Token,
) -> DepositAddress {
    let id_bytes = poseidon_identity.as_bytes();     // Already Poseidon(pubkey)
    let token_bytes = token_type.as_bytes();         // e.g., "BTC" → [66, 84, 67]

    let combined = [id_bytes, token_bytes].concat(); // Poseidon(pubkey) || token_type

    let hashed = PoseidonHasher::hash(&combined);    // Result: 32-byte hash

    DepositAddress::from_hash(hashed)                // Convert to chain-specific format
}
