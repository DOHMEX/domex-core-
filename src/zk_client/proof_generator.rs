// ==================================================
// proof_generator.rs — Domex zk-client Proof Generator (Plonky2 + Poseidon)
// ==================================================

use crate::zk_client::circuit_interface::run_zk_prover;
use crate::zk_client::client_identity::compute_identity_hash;
use crate::types::zk_client::{ZkOnboardingPublicInputs, ZkPrivateInput, ZkOnboardingRequest};
use plonky2::field::goldilocks_field::GoldilocksField;
use crate::poseidon_utils::{u64_to_field, bytes32_to_field};

/// Generates a ZK onboarding proof linking a secret key to a vault and zk-node identity.
/// Uses Plonky2-friendly GoldilocksField and Poseidon for all inputs.
pub fn generate_onboarding_proof(
    private: &ZkPrivateInput,
    vault_id: u64,
    zk_node_id: [u8; 32],
    deposit_chain: &str,
    deposit_tx_hash: &str,
) -> ZkOnboardingRequest {
    // === Step 1: Derive identity hash from (sk || vault_id || node_id)
    let identity_hash: GoldilocksField = compute_identity_hash(&private.sk_bytes, vault_id, &zk_node_id);

    // === Step 2: Build Plonky2-compatible public inputs
    let public_inputs = ZkOnboardingPublicInputs {
        identity_hash,
        vault_id,
        zk_node_id,
        deposit_chain: deposit_chain.to_string(),
        deposit_tx_hash: deposit_tx_hash.to_string(),
    };

    // === Step 3: Generate ZK proof using Plonky2-compatible prover
    let proof = run_zk_prover(private, &public_inputs)
        .expect("Plonky2 zk proof generation failed");

    // === Step 4: Return wrapped onboarding request
    ZkOnboardingRequest {
        proof,
        public_inputs,
    }
}
