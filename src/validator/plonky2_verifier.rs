// ========================================
// ponkey2_verifier.rs — Domex ZK Proof Verifier (Plonky2 + Poseidon)
// ========================================

use crate::types::zk_client::ZkOnboardingPublicInputs;
use crate::types::circuit_interface::Ponkey2ProofBytes;
use crate::poseidon_utils::{string_to_fp, u64_to_fp, bytes_to_fp};

use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::plonk::config::GenericConfig;
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::plonk::config::PoseidonGoldilocksConfig;

/// Verifies a Domex onboarding proof using Plonky2 over Goldilocks + Poseidon
pub fn verify_ponkey2_proof(
    proof: &Ponkey2ProofBytes,
    public_inputs: &ZkOnboardingPublicInputs,
) -> Result<(), String> {
    type F = GoldilocksField;
    type C = PoseidonGoldilocksConfig;

    // === Map inputs into Goldilocks field ===
    let identity_hash_fp = u64_to_fp::<F>(public_inputs.identity_hash.into());
    let vault_id_fp = u64_to_fp::<F>(public_inputs.vault_id);
    let token_fp = string_to_fp::<F>(&public_inputs.token);
    let owner_fp = public_inputs.owner_fp;
    let zk_node_fp = public_inputs.zk_node_fp;

    let expected_public_inputs: Vec<F> = vec![
        identity_hash_fp,
        vault_id_fp,
        token_fp,
        owner_fp,
        zk_node_fp,
    ];

    // === Deserialize proof ===
    let deserialized_proof: ProofWithPublicInputs<F, C> = bincode::deserialize(&proof.bytes)
        .map_err(|e| format!("Failed to deserialize Plonky2 proof: {:?}", e))?;

    // === Verify ===
    let verifier_result = deserialized_proof.verify::<C>();

    match verifier_result {
        Ok(()) => {
            // Optional: Check public inputs match expected
            if deserialized_proof.public_inputs != expected_public_inputs {
                Err("Plonky2 proof public inputs mismatch".to_string())
            } else {
                Ok(())
            }
        }
        Err(e) => Err(format!("Plonky2 proof failed verification: {:?}", e)),
    }
}
