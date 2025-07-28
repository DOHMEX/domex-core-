// Domex :: types/zk_challenge_seed.rs
// Optional shared types for challenge seed logic

use serde::{Serialize, Deserialize};

/// Represents a challenge seed tied to a specific epoch.
/// Can be used in ZK proof circuits, attestation logic, or DAO validation audits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochSeed {
    pub epoch_number: u64,
    pub seed_hash: String,
}
