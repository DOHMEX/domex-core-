// src/validator/global_validator.rs

use crate::validator::{ inbound_proof_handler::handle_incoming_proof, zk_verifier::verify_zk_proof, attestation::build_attestation, proof_attestation::aggregate_attestations, attestation_checker::check_attestation_quorum, merkle_root_finalizer::finalize_merkle_root, bftcomet_rotation::rotate_committee, validator_selection::select_external_validator, epoch_seed::generate_epoch_seed, };

use crate::types::{ZKProofSubmission, AttestationResult, MerkleRootUpdate};

pub fn process_proof_submission(submission: ZKProofSubmission) -> Result<MerkleRootUpdate, String> { // Step 1: Handle incoming proof (deserialize + basic validation) let proof = handle_incoming_proof(submission)?;

// Step 2: Rotate the 300 validator committee via BFTComet
let epoch_seed = generate_epoch_seed();
let committee_300 = rotate_committee(&epoch_seed)?;

// Step 3: Majority picks 1 external validator
let external_validator = select_external_validator(&committee_300, &epoch_seed)?;

// Step 4: All 301 validators verify the ZK proof using Plonky2 backend
let zk_verified_output = verify_zk_proof(&proof)?;

// Step 5: Each validator builds a Poseidon-based proof attestation
let attestation = build_attestation(&zk_verified_output, &proof)?;

// Step 6: Aggregate all 301 attestations using PastaCalcve (batched)
let aggregate_result = aggregate_attestations(attestation, &committee_300, &external_validator)?;

// Step 7: Ensure quorum — all 301 attestations must match
if !check_attestation_quorum(&aggregate_result) {
    return Err("Attestation quorum failed: 301 mismatch".into());
}

// Step 8: Finalize Merkle root state update
let new_root = finalize_merkle_root(&zk_verified_output)?;

Ok(new_root)

}

