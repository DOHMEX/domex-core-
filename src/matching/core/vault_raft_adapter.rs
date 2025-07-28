// ==========================================================
// vault_raft_adapter.rs : Raft Commit Hook for Vault Trades
// ==========================================================

use crate::vault_logic::execute_trade;
use crate::zk::proof_dispatch::dispatch_zk_proof;
use crate::types::{OrderInstruction, TradeResult, VaultState, VaultMetadata};

/// Called by Raft when a vault trade is committed (3-of-5 agreement)
pub fn apply_committed_trade(
    state: &mut VaultState,
    order: OrderInstruction,
    meta: &VaultMetadata,
    last_price: u64,
) -> Result<TradeResult, &'static str> {
    // Step 1: Execute trade on committed vault state
    let result = execute_trade(state, order, meta, last_price)?;

    // Step 2: Trigger ZK proof generation (only Raft leader will submit)
    dispatch_zk_proof(result.clone());

    // Step 3: Log for audit trace
    println!(
        "[RAFT] Trade committed in vault {} @ price {}",
        result.vault_id,
        result.executed_price
    );

    Ok(result)
}
