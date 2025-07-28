// src/types/merkle_state.rs

use serde::{Serialize, Deserialize};

/// A single Merkle leaf (key = identity::token, value = balance)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MerkleLeaf {
    pub key: String,
    pub value: String,
}

/// The final Merkle root string (Poseidon hash)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MerkleRoot(pub String);

/// A proof path from leaf to root
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MerkleProof {
    /// Each item = (sibling_hash, is_right)
    pub path: Vec<(String, bool)>,
}
