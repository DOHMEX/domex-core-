// types/zk_onboarding_verifier.rs
// Shared types for validator-side zk onboarding proof verification in Domex

/// Result of zk onboarding verification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OnboardingVerificationResult {
    Valid,
    Invalid(String),  // human-readable reason
}

/// Errors during onboarding zk proof validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkVerifierError {
    InvalidProof,
    IdentityHashMismatch,
    MalformedPublicInputs,
    UnsafeZeroIdentity,
    VerificationBackendError(String),
}
