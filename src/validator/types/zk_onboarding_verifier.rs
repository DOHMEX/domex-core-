// types/zk_onboarding_verifier.rs
// Shared types for validator-side zk onboarding proof verification in Domex

/// Result of zk onboarding verification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OnboardingVerificationResult {
    Valid,
    ValidWithLock, // for onchain_guarded mode
    Invalid(String),  // human-readable reason
}

/// Errors during onboarding zk proof validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkVerifierError {
    /// Proof failed curve-based cryptographic verification
    InvalidProof,

    /// Computed Poseidon(identity) does not match claimed vault identity
    IdentityHashMismatch,

    /// Malformed or missing zk public inputs
    MalformedPublicInputs,

    /// Poseidon identity hash was zero (forbidden edge case)
    UnsafeZeroIdentity,

    /// Withdrawal mode not declared or unknown
    MissingOrInvalidWithdrawalMode,

    /// Lock reference provided in `onchain_guarded` mode is missing or invalid
    InvalidLockScript,

    /// Circuit did not contain destination address or withdraw intent
    MissingWithdrawIntent,

    /// Cryptographic backend error (Plonky2 or other)
    VerificationBackendError(String),
}
