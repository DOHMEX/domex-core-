// src/types/merkle_finalize.rs

use serde::{Deserialize, Serialize};
use super::merkle_state::MerkleRoot;
use super::attestation::ProofAttestation;

/// Represents a finalized Merkle attestation package
/// used for global validator syncing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizedMerklePackage {
    pub merkle_root: MerkleRoot,
    pub attestation: ProofAttestation,
}
