// ===================================
// event_log.rs — Trading Event Hooks
// ===================================

use crate::types::{OrderInstruction, BalanceChange};
use std::time::{SystemTime, UNIX_EPOCH};

/// Core trade event emitted after a successful execution
pub struct TradeEvent {
    pub vault_id: String,
    pub buyer: String,
    pub seller: String,
    pub token: String,
    pub size: u64,
    pub price: u64,
    pub balance_delta: Vec<BalanceChange>,
    pub timestamp: u64,
}

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
