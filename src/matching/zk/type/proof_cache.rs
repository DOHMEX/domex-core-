// ===============================
// types/proof_cache.rs : Metadata for ZK Proof Caching
// ===============================

use serde::{Deserialize, Serialize};
use crate::types::zk::ZkProofInput;

/// Metadata wrapper for cached proof input.
/// Used for recovery, audit, or batch re-proving.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CachedProof {
    pub input: ZkProofInput,      // Full ZK input payload
    pub timestamp: u64,           // Unix timestamp when cached
    pub status: CacheStatus,      // Tracking for future replay/failure
}

/// Enum to track status of cached proof data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CacheStatus {
    Pending,
    Retried(u8),  // Number of times this input was reprocessed
    Confirmed,    // Already proven and sent to validator
    Failed,       // Proof failed during generation
}
