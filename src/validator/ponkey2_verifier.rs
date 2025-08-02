// ========================================
// ponkey2_verifier.rs — Domex ZK Proof Verifier (Ponkey2 + Pasta + Poseidon)
// ========================================

use crate::types::zk_client::ZkOnboardingPublicInputs;
use crate::types::circuit_interface::Ponkey2ProofBytes;
use crate::poseidon_utils::{string_to_fp, u64_to_fp, bytes_to_fp};
use pasta_curves::Fp;

/// Verifies a Domex onboarding proof using the Ponkey2 circuit backend.
/// This enforces full quantum resistance using Poseidon over Pasta fields.
pub fn verify_ponkey2_proof(
    proof: &Ponkey2ProofBytes,
    public_inputs: &ZkOnboardingPublicInputs,
) -> Result<(), String> {
    // === Extract all inputs as variables ===
    let identity_hash_fp: Fp = public_inputs.identity_hash;
    let withdrawal_mode = public_inputs.withdrawal_mode.clone().unwrap_or_default();
    let vault_id_fp = u64_to_fp(public_inputs.vault_id);
    let token_fp = string_to_fp(&public_inputs.token);
    let owner_fp = public_inputs.owner_fp;
    let zk_node_fp = public_inputs.zk_node_fp;

    // === Run native Ponkey2 proof verification ===
    // This assumes your proof is already serialized and passed from client
    let proof_valid = ponkey2_backend::verify(
        &proof.bytes,
        &[
            identity_hash_fp,
            vault_id_fp,
            token_fp,
            owner_fp,
            zk_node_fp,
        ],
    );

    // === Return proof result ===
    match proof_valid {
        true => Ok(()),
        false => Err("Ponkey2 proof failed verification".to_string()),
    }
}
