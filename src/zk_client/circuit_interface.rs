// ===============================
// zk_client/circuit_interface.rs : Domex ZK Prover SDK (Ponkey2 + Pasta)
// ===============================

use crate::types::zk_client::{ZkOnboardingPublicInputs, ZkPrivateInput};
use crate::poseidon_utils::{u64_to_fp, bytes_to_fp};
use ponkey2_backend::generate_ponkey2_onboarding_proof;
use pasta_curves::Fp;

/// ZK proof output type — opaque bytes to be submitted to Domex validators
pub type ZkProofBytes = Vec<u8>;

/// SDK-level errors returned by ZK proof builder
#[derive(Debug)]
pub enum ZkProverError {
    InvalidSecretKey,
    InvalidNodeId,
    ProofGenerationFailed,
}

/// Builds a Ponkey2-based ZK onboarding proof from private & public inputs
pub fn run_zk_prover(
    private_input: &ZkPrivateInput,
    public_input: &ZkOnboardingPublicInputs,
) -> Result<ZkProofBytes, ZkProverError> {
    // ============================
    // 1. Convert private inputs to field elements
    // ============================

    let sk_fp = Fp::from_bytes(&private_input.sk_bytes)
        .ok_or(ZkProverError::InvalidSecretKey)?;

    let vault_fp = u64_to_fp(public_input.vault_id);
    let node_fp = bytes_to_fp(&public_input.zk_node_id);

    // ============================
    // 2. Call Ponkey2 circuit prover (no placeholder)
    // ============================

    let proof_bytes = generate_ponkey2_onboarding_proof(
        sk_fp,
        vault_fp,
        node_fp,
        public_input.identity_hash,
        &public_input.deposit_chain,
        &public_input.deposit_tx_hash,
    ).map_err(|_| ZkProverError::ProofGenerationFailed)?;

    // ============================
    // 3. Return raw proof to submit to Domex validators
    // ============================

    Ok(proof_bytes)
}
