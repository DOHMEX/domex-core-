// src/validator/types/quorum_sync.rs
// Shared types for validator quorum sync results

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkAttestationPackage {
    pub validator_id: String,        // Poseidon hash or public key
    pub attestation_hash: String,   // Poseidon hash over the attestation
    pub zk_root: String,            // ZK-verified state root
    pub vault_id: String,
    pub token: String,
    pub size: u64,
    pub owner_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuorumSyncResult {
    pub zk_root: String,                 // Final root to be used
    pub attestation_hash: String,       // Canonical attestation hash
    pub vault_id: String,
    pub token: String,
    pub size: u64,
    pub validators: Vec<String>,        // List of all validator IDs that agreed
}
