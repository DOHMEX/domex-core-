// Domex Types – Attestation Broadcast Layer

use serde::{Serialize, Deserialize};

/// A broadcast-ready package containing validator attestation + metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkAttestationPackage {
    pub attestation_hash: String,   // Poseidon hash of the attestation
    pub zk_root: String,            // Final zkRoot after proof verification
    pub vault_id: String,           // Vault involved in this attestation
    pub token: String,              // Token being traded
    pub size: u64,                  // Amount confirmed
    pub owner_hash: String,         // Identity of proof submitter
    pub validator_id: String,       // Sender of this attestation
    pub timestamp: u64,             // Epoch time in seconds
}
