// ===============================
// zk/proof_input.rs : Domex ZK Circuit Input Builder (with liquidity delta)
// ===============================

use crate::types::{TradeResult, ZkProofInput, BalanceChange};

/// Builds the ZK proof input struct from a confirmed trade.
pub fn build_proof_input(trade: &TradeResult, total_liquidity: u64) -> ZkProofInput {
    ZkProofInput {
        vault_id: trade.vault_id.clone(),
        token: trade.token.clone(),
        executed_price: trade.executed_price,
        size: trade.size,
        buyer: trade.buyer.clone(),
        seller: trade.seller.clone(),
        delta: trade.balance_delta.clone(),
        total_liquidity, //  liquidity context for delta compliance
    }
}
