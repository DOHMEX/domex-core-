// types/vault_minter.rs
// Shared vault minting types for Domex zk onboarding

use pasta_curves::Fp;

/// Alias for vault identity (Poseidon(sk || vault_id || zk_node_id))
pub type VaultIdentity = Fp;

/// Represents the amount of token minted to a vault
pub type VaultBalance = u64;

/// Error type for vault minting operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VaultMintError {
    DuplicateVault(VaultIdentity),
    InvalidAmount,
}
