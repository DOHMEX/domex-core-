// ==================================================
// proof_generator.rs — Domex zk-client Proof Generator (Plonky2 + Poseidon)
// ==================================================

use crate::zk_client::circuit_interface::run_zk_prover;
use crate::zk_client::client_identity::compute_identity_hash;
use crate::zk_client::delegate_utils::compute_delegate_hash;
use crate::types::zk_client::{ZkOnboardingPublicInputs, ZkPrivateInput, ZkOnboardingRequest};
use plonky2::field::goldilocks_field::GoldilocksField;
use crate::poseidon_utils::{u64_to_field, bytes32_to_field};

/// Generates a ZK onboarding proof linking a secret key to a vault and zk-node identity,
/// and binds it to a delegate (who must be the one submitting the proof).
///
/// Uses Plonky2-friendly GoldilocksField and Poseidon for all inputs.
pub fn generate_onboarding_proof(
    private: &ZkPrivateInput,
    vault_id: u64,
    zk_node_id: [u8; 32],
    deposit_chain: &str,
    deposit_tx_hash: &str,
    delegate_pubkey: &[u8; 32],
) -> ZkOnboardingRequest {
    // === Step 1: Derive Poseidon-based identity hash: Poseidon(sk || vault_id || zk_node_id)
    let identity_hash: GoldilocksField = compute_identity_hash(&private.sk_bytes, vault_id, &zk_node_id);

    // === Step 2: Compute delegate hash: Poseidon(vault_id || delegate_pubkey)
    let delegate_hash: GoldilocksField = compute_delegate_hash(vault_id, delegate_pubkey);

    // === Step 3: Build Plonky2-compatible public inputs
    let public_inputs = ZkOnboardingPublicInputs {
        identity_hash,
        vault_id,
        zk_node_id,
        deposit_chain: deposit_chain.to_string(),
        deposit_tx_hash: deposit_tx_hash.to_string(),
        delegate_hash, // NEW FIELD
    };

    // === Step 4: Generate ZK proof using Plonky2-compatible prover
    let proof = run_zk_prover(private, &public_inputs)
        .expect("Plonky2 zk proof generation failed");

    // === Step 5: Return wrapped onboarding request
    ZkOnboardingRequest {
        proof,
        public_inputs,
    }
}
