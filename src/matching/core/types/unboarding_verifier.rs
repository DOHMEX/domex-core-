// ===================================================================
// types/unboarding_verifier.rs : ZK Entry Proof for Vault Activation
// ===================================================================

use serde::{Deserialize, Serialize};

/// A zero-knowledge proof input submitted by the user
/// to request Phase 2 (vault activation) access.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserZkEntryProof {
    pub vault_id: String,        // Vault the user wants to enter (e.g., BTC/USDT)
    pub token: String,           // Token being onboarded (e.g., dBTC)
    pub poseidon_hash: String,   // Identity hash bound to vault entry
    pub balance: u64,            // User's verified token balance
    pub total_liquidity: u64,    // Snapshot of vault liquidity at time of proof
    pub merkle_leaf: String,     // ZK-verified leaf (Poseidon hash + balance)
    pub merkle_path: Vec<String> // Merkle path to verify inclusion
}
