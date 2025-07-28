//=============================°=====
// types/global_validator.rs
//=============================°=====

use serde::{Deserialize, Serialize};

/// Submitted ZK proof from local node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZKProofSubmission {
    pub vault_id: String,
    pub token: String,
    pub size: u64,
    pub owner_hash: String,
    pub zk_proof_blob: Vec<u8>,     // Raw serialized proof
    pub timestamp: u64,
}

/// Normalized and verified proof data from Plonky2 verifier
#[derive(Debug, Clone)]
pub struct NormalizedProof {
    pub vault_id: String,
    pub token: String,
    pub size: u64,
    pub owner_hash: String,
    pub zk_root: String,
}

/// Aggregated attestation confirmation from 301 validators
#[derive(Debug, Clone)]
pub struct AttestationResult {
    pub zk_root: String,
    pub attestation_hashes: Vec<String>, // 301 unique hashes
    pub committee: Vec<String>,          // 300 validator IDs
    pub external_validator: String,
}

/// Final Merkle state update to be recorded
#[derive(Debug, Clone)]
pub struct MerkleRootUpdate {
    pub new_root: String,
    pub updated_at: u64,
    pub finalized_by: String, // Poseidon identity of leader
}
