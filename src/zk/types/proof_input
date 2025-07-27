// ======================================
// types/zk.rs  Domex ZK ProofInput Type
// ======================================

use serde::{Serialize, Deserialize};

/// Represents the balance change for an identity and token after trade execution.
/// This is typically used to compute Merkle deltas in ZK circuits.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BalanceChange {
    /// Poseidon hash of the identity (vault owner)
    pub identity: String,

    /// Token symbol (e.g., "dBTC", "dUSDT")
    pub token: String,

    /// Amount changed (positive or negative)
    pub delta: i64,
}

/// Main ZK circuit input passed to Plonky2 prover after Raft-committed trade.
///
/// This structure captures all state changes and participants required to
/// generate a valid ZK proof of execution correctness and liquidity compliance.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZkProofInput {
    /// Vault identifier (e.g., "vault-btc-usdt")
    pub vault_id: String,

    /// Token traded (e.g., "dBTC")
    pub token: String,

    /// Price at which trade was executed
    pub executed_price: u64,

    /// Total traded size
    pub size: u64,

    /// Buyer’s Poseidon identity hash
    pub buyer: String,

    /// Seller’s Poseidon identity hash
    pub seller: String,

    /// Balance mutations resulting from the trade
    pub delta: Vec<BalanceChange>,

    /// Total vault liquidity at time of trade
    pub total_liquidity: u64,
}
