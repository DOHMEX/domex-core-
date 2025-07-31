//=============================
// types/global_validator.rs
// Shared types for global validator zk proof coordination in Domex
//=============================

use serde::{Deserialize, Serialize};

/// ZK proof payload submitted by a local node (e.g., CEX, DAO, bot)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZKProofSubmission {
    pub vault_id: String,             // Logical vault to which proof is bound
    pub token: String,                // Token type (e.g., "dBTC", "dETH")
    pub size: u64,                    // Amount involved in onboarding or withdrawal
    pub owner_hash: String,          // Poseidon(identity binding) as hex
    pub zk_proof_blob: Vec<u8>,      // Raw Plonky2 proof bytes (serialized)
    pub timestamp: u64,              // UNIX timestamp (proof freshness bound)
}

/// Normalized zk proof after Plonky2 verification
#[derive(Debug, Clone)]
pub struct NormalizedProof {
    pub vault_id: String,
    pub token: String,
    pub size: u64,
    pub owner_hash: String,          // Canonical Poseidon identity hash
    pub zk_root: String,             // Root of public input commitments
}

/// Aggregated attestation bundle from all 301 validators
#[derive(Debug, Clone)]
pub struct AttestationResult {
    pub zk_root: String,             // Final agreed root from all 301 attestations
    pub attestation_hashes: Vec<String>, // Individual validator attestations (Poseidon commitments)
    pub committee: Vec<String>,           // 300 validator node IDs (by epoch)
    pub external_validator: String,      // 1 randomly selected witness validator
}

/// Final Merkle root update approved by all validators
#[derive(Debug, Clone)]
pub struct MerkleRootUpdate {
    pub new_root: String,           // Final Merkle state hash
    pub updated_at: u64,            // Timestamp of finalization
    pub finalized_by: String,       // Poseidon identity of quorum leader or aggregator
}
