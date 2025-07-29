// proof_generator.rs
// Domex zk client proof generation logic

use pasta_curves::{Fp, pallas::Scalar as Fr};
use crate::zk_client::client_identity::compute_identity_hash;
use crate::types::zk_client::{ZkOnboardingPublicInputs, ZkPrivateInput, ZkOnboardingRequest};
use crate::zk_client::deposit_address::derive_public_key;
use crate::zk_client::circuit_interface::run_zk_prover;
use ponkey2_poseidon::PoseidonHasher;

/// Generates a zk-proof that proves knowledge of sk and link to native deposit
pub fn generate_onboarding_proof(
    private: &ZkPrivateInput,
    vault_id: u64,
    zk_node_id: [u8; 32],
    deposit_chain: &str,
    deposit_tx_hash: &str,
) -> ZkOnboardingRequest {
    // === 1. Derive pubkey from sk
    let (pk_x, pk_y) = derive_public_key(&private.sk_bytes);

    // === 2. Compute Poseidon identity hash
    let identity_hash = compute_identity_hash(&private.sk_bytes, vault_id, &zk_node_id);

    // === 3. Build public inputs
    let public_inputs = ZkOnboardingPublicInputs {
        identity_hash,
        vault_id,
        zk_node_id,
        pk_x,
        pk_y,
        deposit_chain: deposit_chain.to_string(),
        deposit_tx_hash: deposit_tx_hash.to_string(),
    };

    // === 4. Run ZK Prover to get SNARK proof (Groth16 or Plonk)
    let proof = run_zk_prover(private, &public_inputs)
        .expect("zk proof generation failed");

    // === 5. Return wrapped request
    ZkOnboardingRequest {
        proof,
        public_inputs,
    }
}
