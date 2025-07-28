// Domex :: types/zk_batch.rs
// Shared types for ZK batch proof aggregation and final result

use serde::{Deserialize, Serialize};

/// Represents a single ZK proof that has already been verified.
/// Used as the leaf input for Poseidon-based batch aggregation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedProof {
    pub vault_id: String,
    pub token: String,
    pub size: u64,
    pub owner_hash: String,
    pub timestamp: u64,
    pub zk_payload: Vec<u8>, // raw bytes
}

/// Result of aggregating a batch of verified ZK proofs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchAggregateResult {
    pub zk_root: String,          // Final Merkle root
    pub proof_count: u32,         // Number of proofs aggregated
    pub vaults_touched: Vec<String>, // Distinct vault IDs
    pub total_volume: u64,        // Total size across all proofs
}
