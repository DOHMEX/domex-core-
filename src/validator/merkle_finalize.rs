// src/validator/merkle_finalize.rs

// Domex Merkle Finalization Handler
// Builds ZK-proof-compatible Merkle root and prepares validator attestation

use crate::validator::merkle_state::compute_merkle_root;
use crate::validator::proof_attestation::build_attestation;
use crate::types::vault::VaultState;
use crate::types::normalized_proof::NormalizedProof;
use crate::types::attestation::ProofAttestation;
use crate::types::merkle_state::MerkleRoot;

/// Finalizes Merkle root from current vault state and binds it
/// to the proof via Poseidon attestation for quorum broadcast.
///
/// This output is sent to the 301 validator set for consensus verification.
pub fn finalize_merkle_attestation(
    state: &VaultState,
    verified_proof: &NormalizedProof,
) -> (MerkleRoot, ProofAttestation) {
    // Step 1: Compute the Merkle root from vault state entries
    let (merkle_root, _proofs) = compute_merkle_root(state);

    // Step 2: Build Poseidon-signed attestation for the zk_root
    // This binds validator identity to the verified proof and state transition
    let attestation = build_attestation(
        verified_proof,
        &merkle_root.0, // raw root hash as string or field element
    );

    // Step 3: Return the Merkle root and attestation for validator quorum aggregation
    (merkle_root, attestation)
}
