// ===============================
// types/event_log.rs — Shared Trade Event Types
// ===============================

use crate::types::BalanceChange;

/// Emitted after every successful trade
#[derive(Debug, Clone)]
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
