// =====================================
// vault_logic.rs — Domex Core Executor
// =====================================

//! This module handles the core trade execution path inside a Domex vault node.
//!
//! It is responsible for:
//! 1. Validating order instructions
//! 2. Enforcing the 2% delta rule (vs global liquidity price)
//! 3. Updating account balances
//! 4. Verifying Poseidon-bound identity rights
//! 5. Preparing Merkle deltas for Raft proposal
//! 6. Emitting post-trade ownership transitions

use crate::vault_registry::VaultMetadata;
use crate::identity::verify_poseidon_auth;
use crate::ownership::transfer_ownership;
use crate::delta_checker::check_price_delta;
use crate::balance_snapshot::generate_balance_delta;
use crate::event_log::emit_trade_event;

use crate::types::{OrderInstruction, TradeResult, VaultState, BalanceChange};

/// Executes a trade within a vault given a validated order instruction.
pub fn execute_trade(
    state: &mut VaultState,
    order: OrderInstruction,
    vault_meta: &VaultMetadata,
) -> Result<TradeResult, &'static str> {
    // Step 1: Verify the Poseidon identity matches vault ownership
    if !verify_poseidon_auth(&order.owner_hash, &state.vault_id) {
        return Err("Invalid Poseidon identity for this vault");
    }

    // Step 2: Enforce the 2% delta rule based on global liquidity price
    if !check_price_delta(order.price, vault_meta.liquidity_price, vault_meta.max_delta_bps) {
        return Err("Order violates global liquidity delta rule");
    }

    // Step 3: Check for sufficient balance (basic pre-trade risk check)
    let balance = state.get_balance(&order.owner_hash, &order.token);
    if balance < order.size {
        return Err("Insufficient balance");
    }

    // Step 4: Apply balance mutation (debit from seller, credit to buyer)
    let balance_changes = apply_balance_mutation(state, &order)?;

    // Step 5: Transfer ownership if trade is successful
    transfer_ownership(state, &order.owner_hash, &order.counterparty_hash, &order.token, order.size);

    // Step 6: Generate Merkle-compatible delta
    let delta = generate_balance_delta(&balance_changes);

    // Step 7: Emit trade event for Raft trace
    emit_trade_event(&order, &delta);

    Ok(TradeResult {
        vault_id: state.vault_id.clone(),
        executed_price: order.price,
        buyer: order.counterparty_hash.clone(),
        seller: order.owner_hash.clone(),
        token: order.token.clone(),
        size: order.size,
        balance_delta: delta,
    })
}

/// Handles mutation of internal vault balances (debit + credit).
fn apply_balance_mutation(
    state: &mut VaultState,
    order: &OrderInstruction,
) -> Result<Vec<BalanceChange>, &'static str> {
    let from = &order.owner_hash;
    let to = &order.counterparty_hash;

    // Debit sender
    let sender_balance = state.get_balance(from, &order.token);
    if sender_balance < order.size {
        return Err("Insufficient funds during mutation");
    }
    state.decrease_balance(from, &order.token, order.size);

    // Credit receiver
    state.increase_balance(to, &order.token, order.size);

    Ok(vec![
        BalanceChange {
            identity: from.clone(),
            token: order.token.clone(),
            delta: -(order.size as i64),
        },
        BalanceChange {
            identity: to.clone(),
            token: order.token.clone(),
            delta: order.size as i64,
        },
    ])
}
