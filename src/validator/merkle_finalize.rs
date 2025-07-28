// src/validator/merkle_finalize.rs

// Domex Merkle Finalization Handler
// Builds ZK-proof-compatible Merkle root and prepares final attestation

use crate::validator::merkle_state::compute_merkle_root;
use crate::validator::proof_attestation::build_attestation;
use crate::types::vault::VaultState;
use crate::types::normalized_proof::NormalizedProof;
use crate::types::attestation::ProofAttestation;
use crate::types::merkle_state::MerkleRoot;

/// Computes final Merkle root from vault state and builds attestation
/// to be shared across 301 validator set
pub fn finalize_merkle_attestation(
    state: &VaultState,
    verified_proof: &NormalizedProof,
) -> (MerkleRoot, ProofAttestation) {
    // 1. Build Merkle root from vault state
    let (merkle_root, _) = compute_merkle_root(state);

    // 2. Build cryptographic attestation from proof and root
    let attestation = build_attestation(verified_proof, &merkle_root.0);

    // 3. Return both for quorum broadcast
    (merkle_root, attestation)
}
