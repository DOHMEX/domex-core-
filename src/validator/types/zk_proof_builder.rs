// src/types/zk_proof_builder.rs

use serde::{Deserialize, Serialize};

/// Represents the final attestation created after processing a ZK proof.
/// This struct is used by the proof builder module to package proof metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofAttestation {
    /// Vault identifier this proof is associated with
    pub vault_id: String,

    /// Token symbol involved in the proof (e.g., "dBTC")
    pub token: String,

    /// Quantity or size covered by the proof
    pub size: u64,

    /// Poseidon hash representing the vault owner’s identity
    pub owner_hash: String,

    /// Merkle root or zk root after proof processing
    pub zk_root: String,

    /// Poseidon hash commitment over all attestation fields (unique ID)
    pub attestation_hash: String,

    /// Unix timestamp (seconds) marking attestation creation time
    pub timestamp: u64,
}
