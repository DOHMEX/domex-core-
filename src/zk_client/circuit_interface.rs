// ===============================
// circuit_interface.rs — Domex ZK Prover SDK (Plonky2 + Goldilocks + Poseidon)
// ===============================

use crate::types::zk_client::{ZkOnboardingPublicInputs, ZkPrivateInput};
use crate::poseidon_utils::{u64_to_goldilocks, bytes_to_goldilocks};
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2_backend::generate_ponkey2_onboarding_proof;

/// Output type: Raw proof bytes to be submitted to Domex global validators
pub type ZkProofBytes = Vec<u8>;

/// ZK Prover SDK error types
#[derive(Debug)]
pub enum ZkProverError {
    InvalidSecretKey,
    InvalidNodeId,
    ProofGenerationFailed,
}

/// Runs the Domex ZK onboarding circuit using Plonky2 (Goldilocks + Poseidon)
/// Binds: Poseidon(sk || vault_id || zk_node_id) → identity_hash
///
/// # Arguments:
/// - `private_input`: ZkPrivateInput containing 32-byte secret key
/// - `public_input`: ZkOnboardingPublicInputs (vault_id, node_id, identity_hash, etc.)
///
/// # Returns:
/// - Ok(Vec<u8>) if proof generation is successful
/// - Err(ZkProverError) otherwise
pub fn run_zk_prover(
    private_input: &ZkPrivateInput,
    public_input: &ZkOnboardingPublicInputs,
) -> Result<ZkProofBytes, ZkProverError> {
    // Step 1: Convert secret key to Goldilocks field
    let sk_slice = private_input.sk_bytes.get(..8)
        .ok_or(ZkProverError::InvalidSecretKey)?;

    let sk_fp = GoldilocksField::from_canonical_u64(
        u64::from_le_bytes(sk_slice.try_into().map_err(|_| ZkProverError::InvalidSecretKey)?)
    );

    // Step 2: Convert vault ID and node ID
    let vault_fp = u64_to_goldilocks(public_input.vault_id);
    let node_fp = bytes_to_goldilocks(&public_input.zk_node_id);

    // Step 3: Generate proof using Plonky2 backend
    let proof_bytes = generate_ponkey2_onboarding_proof(
        sk_fp,
        vault_fp,
        node_fp,
        public_input.identity_hash,
        &public_input.deposit_chain,
        &public_input.deposit_tx_hash,
    ).map_err(|_| ZkProverError::ProofGenerationFailed)?;

    Ok(proof_bytes)
}
