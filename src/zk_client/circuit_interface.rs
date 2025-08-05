// ===============================
// zk_client/circuit_interface.rs : Domex ZK Prover SDK (Plonky2 + Goldilocks)
// ===============================

use crate::types::zk_client::{ZkOnboardingPublicInputs, ZkPrivateInput};
use crate::poseidon_utils::{u64_to_goldilocks, bytes_to_goldilocks};
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2_backend::generate_ponkey2_onboarding_proof;

/// Raw ZK proof bytes to be submitted to Domex global validators
pub type ZkProofBytes = Vec<u8>;

/// SDK-level errors returned by ZK proof generation pipeline
#[derive(Debug)]
pub enum ZkProverError {
    InvalidSecretKey,
    InvalidNodeId,
    ProofGenerationFailed,
}

/// Runs the Plonky2-based onboarding circuit to generate a proof binding:
/// (secret key || vault ID || node ID) → identity_hash
///
/// # Returns:
/// - Ok(Vec<u8>) = Serialized ZK proof to be sent to validators
/// - Err(ZkProverError) = On failure (bad input, circuit error)
pub fn run_zk_prover(
    private_input: &ZkPrivateInput,
    public_input: &ZkOnboardingPublicInputs,
) -> Result<ZkProofBytes, ZkProverError> {
    // ============================
    // 1. Convert inputs to Plonky2-compatible field elements
    // ============================

    let sk_fp = match private_input.sk_bytes.get(..8) {
        Some(slice) => GoldilocksField::from_canonical_u64(u64::from_le_bytes(slice.try_into().unwrap())),
        None => return Err(ZkProverError::InvalidSecretKey),
    };

    let vault_fp = u64_to_goldilocks(public_input.vault_id);
    let node_fp = bytes_to_goldilocks(&public_input.zk_node_id);

    // ============================
    // 2. Call the circuit backend (Plonky2 over Goldilocks + Poseidon)
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
    // 3. Return ZK proof
    // ============================

    Ok(proof_bytes)
}
