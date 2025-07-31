//=======================================
// Domex :: types/proof_attestation.rs
// Validator attestation struct for zk batch Merkle root confirmation
//=======================================

use serde::{Deserialize, Serialize};

/// A validator’s signed attestation over a zk_root (Merkle batch of proofs).
/// Signed using Poseidon-hashed commitment to preserve cryptographic identity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofAttestation {
    pub zk_root: String,             // Aggregated Merkle root of the zk proof batch
    pub validator_id: String,        // Poseidon hash of validator identity (sk + node_id)
    pub signature: String,           // Signature over zk_root commitment (Poseidon or Groth16-based)
    pub proof_count: u32,            // Number of ZK proofs in the batch
    pub total_volume: u64,           // Total token volume involved in all verified actions
    pub vaults_touched: Vec<String>, // Unique vaults impacted in this batch
    pub attested_at: u64,            // UNIX timestamp of when the attestation was finalized
}
