// Domex :: types/proof_attestation.rs
// Struct representing a signed validator attestation of a zk_root batch

use serde::{Deserialize, Serialize};

/// An attestation from a validator confirming agreement with a ZK batch result.
/// Signed using Poseidon-hashed metadata to ensure identity binding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofAttestation {
    pub zk_root: String,             // Merkle root of aggregated ZK proofs
    pub validator_id: String,        // Identity hash of the validator
    pub signature: String,           // Poseidon-signed commitment
    pub proof_count: u32,            // Number of proofs included in the batch
    pub total_volume: u64,           // Combined trade volume from all proofs
    pub vaults_touched: Vec<String>, // All vaults involved in the batch
    pub attested_at: u64,            // UNIX timestamp of attestation
}
