//========================================
// src/validator/global_validator.rs
// Domex global validator flow for zk proof processing and state updates
//========================================

use crate::validator::{
    attestation::build_attestation,
    attestation_checker::check_attestation_quorum,
    bftcomet_rotation::rotate_committee,
    epoch_seed::generate_epoch_seed,
    inbound_proof_handler::handle_incoming_proof,
    merkle_root_finalizer::finalize_merkle_root,
    proof_attestation::aggregate_attestations,
    validator_selection::select_external_validator,
    zk_verifier::verify_zk_proof,
};

use crate::types::{ZKProofSubmission, AttestationResult, MerkleRootUpdate};

/// Process incoming zk proof submission and update Merkle root if valid
pub fn process_proof_submission(
    submission: ZKProofSubmission,
) -> Result<MerkleRootUpdate, String> {
    // Step 1: Deserialize and validate incoming proof payload
    let proof = handle_incoming_proof(submission)?;

    // Step 2: Generate epoch seed and rotate 300 validator committee
    let epoch_seed = generate_epoch_seed();
    let committee_300 = rotate_committee(&epoch_seed)?;

    // Step 3: Select external validator to act as quorum witness
    let external_validator = select_external_validator(&committee_300, &epoch_seed)?;

    // Step 4: Perform zk proof verification using Plonky2 (via zk_verifier)
    let zk_verified_output = verify_zk_proof(&proof)?;

    // Step 5: Build local attestation with Poseidon commitment to proof output
    let attestation = build_attestation(&zk_verified_output, &proof)?;

    // Step 6: Aggregate all 301 validator attestations using PastaCalcve
    let aggregate_result = aggregate_attestations(
        attestation,
        &committee_300,
        &external_validator,
    )?;

    // Step 7: Ensure strict quorum (301/301 must match)
    if !check_attestation_quorum(&aggregate_result) {
        return Err("Attestation quorum failed: 301 mismatch".into());
    }

    // Step 8: Finalize and return updated Merkle root
    let new_root = finalize_merkle_root(&zk_verified_output)?;

    Ok(new_root)
}
