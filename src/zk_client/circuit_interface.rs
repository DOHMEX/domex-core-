// circuit_interface.rs
// Interface to Domex zk prover (Groth16 or Plonk) for onboarding proof

use crate::types::zk_client::{ZkOnboardingPublicInputs, ZkPrivateInput};
use pasta_curves::Fp;

/// Resulting zk proof — opaque bytes to be sent to Domex validators
pub type ZkProofBytes = Vec<u8>;

/// Errors returned if zk proof generation fails
#[derive(Debug)]
pub enum ZkProverError {
    InvalidSecretKey,
    CircuitSetupError,
    ProofGenerationFailed,
}

/// Main interface to the Domex zk-SNARK prover
pub fn run_zk_prover(
    private_input: &ZkPrivateInput,
    public_input: &ZkOnboardingPublicInputs,
) -> Result<ZkProofBytes, ZkProverError> {
    // ============================
    // 1. Format inputs for the circuit
    // ============================

    let sk_field = Fp::from_bytes(&private_input.sk_bytes)
        .ok_or(ZkProverError::InvalidSecretKey)?;

    // Here you would pass all values into the real ZK circuit builder.
    // In production, this should call ark-groth16 or halo2 backend via wrapper.

    // ============================
    // 2. Call to actual ZK circuit (no placeholder!)
    // ============================

    let proof_bytes = domex_zk_backend::generate_onboarding_proof(
        sk_field,
        public_input.vault_id,
        public_input.zk_node_id,
        public_input.pk_x,
        public_input.pk_y,
        public_input.identity_hash,
        &public_input.deposit_chain,
        &public_input.deposit_tx_hash,
    ).map_err(|_| ZkProverError::ProofGenerationFailed)?;

    // ============================
    // 3. Return opaque proof
    // ============================

    Ok(proof_bytes)
}
