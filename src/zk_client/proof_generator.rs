// ==============================================
// proof_generator.rs — Domex zk-client Proof Generator (Ponkey2 + Pasta)
// ==============================================

use pasta_curves::{Fp, pallas::Scalar as Fr};
use crate::zk_client::client_identity::compute_identity_hash;
use crate::zk_client::deposit_address::derive_public_key;
use crate::zk_client::circuit_interface::run_zk_prover;
use crate::types::zk_client::{ZkOnboardingPublicInputs, ZkPrivateInput, ZkOnboardingRequest};

/// Generates a ZK onboarding proof linking user's secret key to vault & deposit.
/// This proof binds: (1) identity hash, (2) public key, and (3) zk-node metadata.
///
/// Returns a ready-to-send ZkOnboardingRequest.
pub fn generate_onboarding_proof(
    private: &ZkPrivateInput,
    vault_id: u64,
    zk_node_id: [u8; 32],
    deposit_chain: &str,
    deposit_tx_hash: &str,
) -> ZkOnboardingRequest {
    // === Step 1: Derive public key from secret key ===
    let (pk_x, pk_y) = derive_public_key(&private.sk_bytes);

    // === Step 2: Compute Poseidon(sk || vault_id || zk_node_id) ===
    let identity_hash = compute_identity_hash(&private.sk_bytes, vault_id, &zk_node_id);

    // === Step 3: Build public inputs ===
    let public_inputs = ZkOnboardingPublicInputs {
        identity_hash,
        vault_id,
        zk_node_id,
        pk_x,
        pk_y,
        deposit_chain: deposit_chain.to_owned(),
        deposit_tx_hash: deposit_tx_hash.to_owned(),
    };

    // === Step 4: Call internal Ponkey2 circuit prover ===
    let proof = run_zk_prover(private, &public_inputs)
        .expect("zk proof generation failed");

    // === Step 5: Return onboarding request object ===
    ZkOnboardingRequest {
        proof,
        public_inputs,
    }
}
