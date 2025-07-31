// src/types/merkle_finalize.rs

use serde::{Deserialize, Serialize};
use super::merkle_state::MerkleRoot;
use super::attestation::ProofAttestation;

/// Represents a finalized Merkle attestation package
/// prepared after validator quorum agreement and ZK root validation.
///
/// This structure is broadcasted across the global validator network
/// to synchronize final state roots with attested proof metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizedMerklePackage {
    pub merkle_root: MerkleRoot,           // Final Merkle root after zk-proof batch
    pub attestation: ProofAttestation,     // Poseidon-signed validator attestation
}
