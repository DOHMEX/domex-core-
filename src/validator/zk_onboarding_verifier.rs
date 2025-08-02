// ===============================
// zk_onboarding_verifier.rs — Domex ZK Onboarding Verifier (Ponkey2 + Pasta)
// ===============================

use crate::types::zk_client::ZkOnboardingPublicInputs;
use crate::types::circuit_interface::{Ponkey2ProofBytes, CircuitInputs};
use crate::validator::ponkey2_verifier::verify_ponkey2_proof;
use crate::hash_utils::{poseidon_hash, u64_to_fp, bytes_to_fp};
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

/// Verifies ZK onboarding proof using Ponkey2 + Pasta + Poseidon
pub fn verify_onboarding_proof(
    proof_bytes: &Ponkey2ProofBytes,
    public_inputs: &ZkOnboardingPublicInputs,
) -> Result<OnboardingVerificationResult, ZkVerifierError> {
    // === Phase 1: Input checks ===
    if public_inputs.identity_hash.is_zero() {
        return Err(ZkVerifierError::UnsafeZeroIdentity);
    }

    if public_inputs.withdrawal_mode.is_none() {
        return Err(ZkVerifierError::MissingOrInvalidWithdrawalMode);
    }

    // === Phase 2: Native Ponkey2 proof validation ===
    verify_ponkey2_proof(proof_bytes, public_inputs)
        .map_err(|e| ZkVerifierError::VerificationBackendError(format!("{:?}", e)))?;

    // === Phase 3: Withdrawal rules ===
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
