// ===============================
// zk_client/circuit_interface.rs : Domex ZK Prover SDK (Plonky2 + Goldilocks)
// ===============================

use crate::types::zk_client::{ZkOnboardingPublicInputs, ZkPrivateInput};
use crate::poseidon_utils::{u64_to_goldilocks, bytes_to_goldilocks};
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2_backend::generate_ponkey2_onboarding_proof;

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
    // 1. Convert private inputs to Goldilocks field elements
    // ============================

    let sk_fp = GoldilocksField::from_canonical_u64(
        u64::from_le_bytes(private_input.sk_bytes[..8].try_into().unwrap_or_default())
    );

    let vault_fp = u64_to_goldilocks(public_input.vault_id);
    let node_fp = bytes_to_goldilocks(&public_input.zk_node_id);

    // ============================
    // 2. Call Ponkey2 circuit prover (Plonky2 over Goldilocks)
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
