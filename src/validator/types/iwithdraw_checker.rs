// types/withdraw_checker.rs
// Shared types for Domex withdrawal validation and identity-bound proof logic

use serde::{Deserialize, Serialize};
use pasta_curves::Fp;

/// Represents a withdrawal request submitted for validator-side review
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithdrawalRequest {
    pub vault_id: String,
    pub token: String,
    pub amount: u64,
    pub identity_hash: Fp,
    pub zk_proof_blob: Vec<u8>,
    pub timestamp: u64,
}

/// Result of checking if the withdrawal is allowed
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WithdrawalStatus {
    Approved,
    Rejected(String),
}

/// Error types for withdrawal validation logic
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WithdrawalCheckError {
    VaultNotFound,
    InsufficientBalance,
    IdentityHashMismatch,
    ZkProofInvalid,
    ProofFormatError,
    ZeroIdentity,
}
