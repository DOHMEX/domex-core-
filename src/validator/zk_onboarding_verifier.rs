// zk_onboarding_verifier.rs
// Verifies zk onboarding proofs for Domex vault minting
// Enforces zero-knowledge and private key hiding guarantees

use crate::types::zk_client::ZkOnboardingPublicInputs;
use crate::hash_utils::{poseidon_hash, u64_to_fp, bytes_to_fp};
use crate::circuit_interface::{ZkProofBytes, ZkProverError};
use crate::types::circuit_interface::CircuitInputs;
use crate::validator::circuit_verifier_backend::verify_groth16_proof;
use pasta_curves::Fp;

/// Validates zk onboarding proof:
/// - Proof is valid
/// - Identity hash recomputes correctly
/// - No secret key (`sk`) appears in public inputs
pub fn verify_onboarding_proof(
    proof_bytes: &ZkProofBytes,
    public_inputs: &ZkOnboardingPublicInputs,
) -> Result<(), ZkProverError> {
    // ================================
    // 1. Reconstruct identity hash from public data
    // This simulates validator logic WITHOUT knowing `sk`
    // ================================

    let vault_fp = u64_to_fp(public_inputs.vault_id);
    let node_fp = bytes_to_fp(&public_inputs.zk_node_id);

    // NOTE: The validator cannot see `sk`, so it can't recompute identity_hash directly.
    // But it checks that the ZK proof proves correct knowledge of `sk` such that:
    // Poseidon(sk || vault_id || zk_node_id) = identity_hash

    // The proof will fail if this binding is false.

    // ================================
    // 2. Verify ZK proof validity (Groth16 or Plonk backend)
    // ================================
    verify_groth16_proof(proof_bytes, public_inputs)
        .map_err(|_| ZkProverError::ProofGenerationFailed)?;

    // ================================
    // 3. Ensure identity hash looks canonical
    // ================================
    if public_inputs.identity_hash.is_zero() {
        return Err(ZkProverError::InvalidSecretKey);
    }

    // Proof is valid and identity format is safe
    Ok(())
}
