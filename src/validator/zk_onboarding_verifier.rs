// ===============================
// zk_onboarding_verifier.rs — Domex ZK Onboarding Verifier (Ponkey2 + Pasta)
// ===============================

use crate::types::zk_client::ZkOnboardingPublicInputs;
use crate::hash_utils::{poseidon_hash, u64_to_fp, bytes_to_fp};
use crate::types::circuit_interface::{ZkProofBytes, CircuitInputs};
use crate::validator::circuit_verifier_backend::verify_groth16_proof;
use pasta_curves::Fp;

/// Final result of a ZK onboarding verification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OnboardingVerificationResult {
    Valid,
    ValidWithLock, // Used for script-locked vaults (e.g., BTC)
    Invalid(String),
}

/// ZK verifier errors for onboarding
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkVerifierError {
    InvalidProof,
    IdentityHashMismatch,
    UnsafeZeroIdentity,
    MissingOrInvalidWithdrawalMode,
    InvalidLockScript,
    MissingWithdrawIntent,
    VerificationBackendError(String),
}

/// Verifies ZK onboarding proof using Groth16 + Pasta + Ponkey2 (quantum-safe)
pub fn verify_onboarding_proof(
    proof_bytes: &ZkProofBytes,
    public_inputs: &ZkOnboardingPublicInputs,
) -> Result<OnboardingVerificationResult, ZkVerifierError> {
    // === Phase 1: Public input validation ===
    if public_inputs.identity_hash.is_zero() {
        return Err(ZkVerifierError::UnsafeZeroIdentity);
    }

    if public_inputs.withdrawal_mode.is_none() {
        return Err(ZkVerifierError::MissingOrInvalidWithdrawalMode);
    }

    // === Phase 2: Proof validation (Groth16 over Ponkey2 circuits) ===
    verify_groth16_proof(proof_bytes, public_inputs)
        .map_err(|e| ZkVerifierError::VerificationBackendError(format!("{:?}", e)))?;

    // === Phase 3: Mode-specific checks ===
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
