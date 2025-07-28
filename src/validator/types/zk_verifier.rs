// Domex :: types/zk_verifier.rs
// Types used for ZK verification logic within global validators

use serde::{Serialize, Deserialize};

/// Payload to be sent into the external Plonky2-compatible verifier binary.
/// This structure is serialized to JSON before being piped into the verifier.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZkVerificationInput {
    pub vault_id: String,
    pub token: String,
    pub size: u64,
    pub owner_hash: String,
    pub timestamp: u64,
    pub zk_payload: String, // base64-encoded
}

/// Output returned from the external ZK verifier binary.
/// Can be expanded in future to include proof metadata.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZkVerificationOutput {
    pub zk_root: String,
}
