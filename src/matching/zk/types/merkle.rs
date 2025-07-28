// ==========================================
// types/merkle.rs : Domex Merkle Structures
// ========================================

use serde::{Serialize, Deserialize};

/// Represents a Merkle root state for a vault before and after an action (e.g., trade, exit).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MerkleDelta {
    /// The vault Merkle root before the transition (e.g., before a trade or withdrawal)
    pub before_root: String,

    /// The vault Merkle root after the transition
    pub after_root: String,

    /// Optional hash of the affected leaf (e.g., a vault balance change)
    pub leaf_hash: Option<String>,
}

/// A single step in a Merkle proof path, from leaf to root.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MerklePathStep {
    /// Hash of the sibling node at this level
    pub sibling_hash: String,

    /// Whether the current node is on the left or right (true = left, false = right)
    pub is_left: bool,
}

/// A full Merkle proof from leaf to root, used for ZK onboarding verification
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MerkleProof {
    /// The leaf hash (e.g., Poseidon hash of vault entry)
    pub leaf: String,

    /// Path of sibling hashes to compute the root
    pub path: Vec<MerklePathStep>,

    /// Claimed Merkle root (should match global state root)
    pub expected_root: String,
}
