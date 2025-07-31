// types/vault_minter.rs
// Shared vault minting and withdrawal types for Domex zk onboarding & exit

use pasta_curves::Fp;

/// Alias for vault identity (Poseidon(sk || vault_id || zk_node_id))
pub type VaultIdentity = Fp;

/// Represents the amount of token minted to a vault
pub type VaultBalance = u64;

/// Enum for vault intent (minting or withdrawal)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VaultActionType {
    Mint,
    Withdraw,
}

/// Result enum used to confirm success or failure of an action
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VaultActionResult {
    Success,
    Failure(VaultActionError),
}

/// Unified error type for vault minting and withdrawal operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VaultActionError {
    /// Tried to onboard again to an existing vault
    DuplicateVault(VaultIdentity),
    /// Provided token amount was zero or invalid
    InvalidAmount,
    /// Attempted to withdraw without valid proof
    UnauthorizedWithdraw(VaultIdentity),
    /// Tried to withdraw more than available balance
    InsufficientBalance(VaultIdentity, VaultBalance),
}
