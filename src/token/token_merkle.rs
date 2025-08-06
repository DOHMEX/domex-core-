// ==========================================================
// token_merkle.rs — DOMEX Poseidon-Based Token Merkle Tree
// ==========================================================
//
// Maintains Merkle root of DOMEX token state.
// Used in validator attestation, onboarding, withdrawal proofs.
//

use poseidon_utils::{poseidon_hash, PoseidonLeaf};
use std::collections::BTreeMap;

/// Represents a user token balance entry to be inserted into the Merkle tree.
#[derive(Debug, Clone)]
pub struct TokenLeaf {
    pub vault_id: String,
    pub owner: String, // Poseidon hash of public key
    pub balance: u64,
}

impl TokenLeaf {
    pub fn to_poseidon_leaf(&self) -> PoseidonLeaf {
        PoseidonLeaf::new(vec![
            poseidon_hash(&[self.vault_id.as_bytes()]),
            poseidon_hash(&[self.owner.as_bytes()]),
            poseidon_hash(&[self.balance.to_be_bytes()]),
        ])
    }
}

/// Merkle tree over token balances.
/// Stores sorted leaves and updates root when needed.
#[derive(Debug)]
pub struct TokenMerkle {
    pub leaves: BTreeMap<String, TokenLeaf>, // key = vault_id + owner
    pub root: [u8; 32],
}

impl TokenMerkle {
    pub fn new() -> Self {
        TokenMerkle {
            leaves: BTreeMap::new(),
            root: [0u8; 32],
        }
    }

    /// Insert or update a token leaf and recompute Merkle root.
    pub fn insert_or_update(&mut self, leaf: TokenLeaf) {
        let key = format!("{}:{}", leaf.vault_id, leaf.owner);
        self.leaves.insert(key, leaf);
        self.root = self.compute_root();
    }

    /// Returns the Merkle root of current token state.
    pub fn get_root(&self) -> [u8; 32] {
        self.root
    }

    /// Internal: recomputes the Merkle root from current leaves.
    fn compute_root(&self) -> [u8; 32] {
        let mut poseidon_leaves: Vec<PoseidonLeaf> = self
            .leaves
            .values()
            .map(|leaf| leaf.to_poseidon_leaf())
            .collect();

        // Pad to next power of 2 if necessary
        while poseidon_leaves.len() & (poseidon_leaves.len() - 1) != 0 {
            poseidon_leaves.push(PoseidonLeaf::default());
        }

        poseidon_utils::compute_poseidon_merkle_root(poseidon_leaves)
    }

    /// Returns inclusion proof for a given vault + owner.
    pub fn generate_proof(&self, vault_id: &str, owner: &str) -> Option<Vec<[u8; 32]>> {
        let key = format!("{}:{}", vault_id, owner);
        self.leaves.get(&key).map(|leaf| {
            let leaf_hash = leaf.to_poseidon_leaf();
            poseidon_utils::generate_merkle_proof(&leaf_hash, &self.leaves)
        })
    }
}
