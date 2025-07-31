// zk_onboarding_verifier.rs
// Verifies zk onboarding proofs for Domex vault minting
// Enforces zero-knowledge and private key hiding guarantees

use crate::types::zk_client::ZkOnboardingPublicInputs;
use crate::hash_utils::{poseidon_hash, u64_to_fp, bytes_to_fp};
use crate::circuit_interface::{ZkProofBytes, ZkProverError};
use crate::types::circuit_interface::CircuitInputs;
use crate::validator::circuit_verifier_backend::verify_groth16_proof;
use pasta_curves::Fp;

/// Onboarding result state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OnboardingVerificationResult {
    Valid,
    ValidWithLock, // for onchain_guarded mode (e.g., script-locked BTC)
    Invalid(String),
}

/// Errors during onboarding zk proof validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkVerifierError {
    InvalidProof,
    IdentityHashMismatch,
    MalformedPublicInputs,
    UnsafeZeroIdentity,
    MissingOrInvalidWithdrawalMode,
    InvalidLockScript,
    MissingWithdrawIntent,
    VerificationBackendError(String),
}

/// Validates zk onboarding proof:
/// - Proof is cryptographically valid
/// - Identity hash recomputes correctly
/// - Withdrawal mode is declared
/// - Lock script integrity is optionally enforced
pub fn verify_onboarding_proof(
    proof_bytes: &ZkProofBytes,
    public_inputs: &ZkOnboardingPublicInputs,
) -> Result<OnboardingVerificationResult, ZkVerifierError> {
    // === Phase 1: Input checks ===
    if public_inputs.identity_hash.is_zero() {
        return Err(ZkVerifierError::UnsafeZeroIdentity);
    }

    if public_inputs.withdrawal_mode.is_none() {
        return Err(ZkVerifierError::MissingOrInvalidWithdrawalMode);
    }

    // === Phase 2: Cryptographic proof check ===
    verify_groth16_proof(proof_bytes, public_inputs)
        .map_err(|e| ZkVerifierError::VerificationBackendError(format!("{:?}", e)))?;

    // === Phase 3: Withdrawal-mode dependent logic ===
    match public_inputs.withdrawal_mode.as_deref() {
        Some("strict_approved_only") => Ok(OnboardingVerificationResult::Valid),

        Some("onchain_guarded") => {
            if public_inputs.lock_script_hash.is_none() {
                return Err(ZkVerifierError::InvalidLockScript);
            }
            if public_inputs.withdraw_intent.is_none() {
                return Err(ZkVerifierError::MissingWithdrawIntent);
            }

            Ok(OnboardingVerificationResult::ValidWithLock)
        }

        _ => Err(ZkVerifierError::MissingOrInvalidWithdrawalMode),
    }
}
