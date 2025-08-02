// Domex :: zk/zk_onboarding_verifier.rs
// Verifies ZK onboarding proofs for Domex vault minting using Plonky2
// Ensures full quantum resistance and Poseidon identity hashing guarantees

use crate::types::zk_client::ZkOnboardingPublicInputs;
use crate::hash_utils::{poseidon_hash, u64_to_fp, bytes_to_fp};
use crate::types::circuit_interface::{ZkProofBytes, CircuitInputs};
use crate::validator::plonky2_verifier::verify_plonky2_recursive_proof;
use pasta_curves::Fp;

/// Onboarding result state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OnboardingVerificationResult {
    Valid,
    ValidWithLock, // for onchain_guarded mode (e.g., BTC script vaults)
    Invalid(String),
}

/// Errors during ZK onboarding validation
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

/// Fully quantum-resistant onboarding verifier using Plonky2
pub fn verify_onboarding_proof(
    proof_bytes: &ZkProofBytes,
    public_inputs: &ZkOnboardingPublicInputs,
) -> Result<OnboardingVerificationResult, ZkVerifierError> {
    // === Phase 1: Input Consistency Check ===
    if public_inputs.identity_hash.is_zero() {
        return Err(ZkVerifierError::UnsafeZeroIdentity);
    }

    if public_inputs.withdrawal_mode.is_none() {
        return Err(ZkVerifierError::MissingOrInvalidWithdrawalMode);
    }

    // === Phase 2: Quantum-Safe Proof Verification (Plonky2) ===
    verify_plonky2_recursive_proof(proof_bytes, public_inputs)
        .map_err(|e| ZkVerifierError::VerificationBackendError(format!("{:?}", e)))?;

    // === Phase 3: Withdrawal Logic Enforcement ===
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
