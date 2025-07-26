// ===================================
// event_log.rs — Trading Event Hooks
// ===================================

use crate::types::{OrderInstruction, BalanceChange};
use crate::types::event_log::TradeEvent;
use std::time::{SystemTime, UNIX_EPOCH};

/// Emits a trade event to stdout or optional subscriber
pub fn emit_trade_event(order: &OrderInstruction, delta: &Vec<BalanceChange>) {
    let event = TradeEvent {
        vault_id: order.vault_id.clone(),
        buyer: order.counterparty_hash.clone(),
        seller: order.owner_hash.clone(),
        token: order.token.clone(),
        size: order.size,
        price: order.price,
        balance_delta: delta.clone(),
        timestamp: current_unix_timestamp(),
    };

    // For now, we just log to stdout — pluggable to a WebSocket or indexer
    println!(
        "[EVENT] TRADE: vault={} buyer={} seller={} size={} price={} token={}",
        event.vault_id,
        event.buyer,
        event.seller,
        event.size,
        event.price,
        event.token
    );
}

/// Returns the current Unix timestamp
fn current_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
