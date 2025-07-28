// =====================================
// order_book.rs — Domex Order Matcher
// =====================================

//! Matching engine for limit/market orders inside a vault.
//! Applies skip logic (only one node matches), and emits raft proposals.

use crate::types::{OrderInstruction, TradeResult, VaultState, RaftProposal};
use crate::vault_registry::{VaultMetadata, is_vault_active};
use crate::vault_logic::execute_trade;

use std::collections::{BTreeMap, VecDeque};

/// OrderBook stores limit orders for a single token pair.
pub struct OrderBook {
    bids: BTreeMap<u64, VecDeque<OrderInstruction>>, // price -> FIFO queue
    asks: BTreeMap<u64, VecDeque<OrderInstruction>>, // price -> FIFO queue
    last_price: u64,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            last_price: 0,
        }
    }

    /// Submit a new order to the matcher. Returns Raft proposal if matched.
    pub fn submit_order(
        &mut self,
        state: &mut VaultState,
        order: OrderInstruction,
        vault_meta: &VaultMetadata,
    ) -> Option<RaftProposal> {
        // Phase 2 entry validation — must be ZK-verified
        if !is_vault_active(&state.vault_id, &order.owner_hash) {
            println!("[MATCH] Rejected: vault not ZK-activated for {}", &order.owner_hash);
            return None;
        }

        // Skip logic: this node is responsible for matching
        if let Some(counterparty) = self.match_order(&order) {
            let matched_price = order.price;

            let filled_order = OrderInstruction {
                counterparty_hash: counterparty.owner_hash.clone(),
                ..order.clone()
            };

            //  Core vault execution
            match execute_trade(state, filled_order.clone(), vault_meta, self.last_price) {
                Ok(result) => {
                    self.last_price = matched_price;
                    Some(RaftProposal {
                        vault_id: state.vault_id.clone(),
                        trade: result,
                    })
                }
                Err(e) => {
                    println!("[MATCH] Trade execution failed: {}", e);
                    None
                }
            }
        } else {
            // No match: queue as limit order
            self.enqueue_order(order);
            None
        }
    }

    /// Match incoming order against opposite side of the book
    fn match_order(&mut self, order: &OrderInstruction) -> Option<OrderInstruction> {
        let side = order.intent.as_str();
        let book = if side == "buy" {
            &mut self.asks
        } else {
            &mut self.bids
        };

        for (price, queue) in book.iter_mut() {
            let acceptable = if side == "buy" {
                order.price >= *price
            } else {
                order.price <= *price
            };

            if acceptable && !queue.is_empty() {
                let matched = queue.pop_front();
                return matched;
            }
        }
        None
    }

    /// Queue a limit order if not matched
    fn enqueue_order(&mut self, order: OrderInstruction) {
        let book = if order.intent == "buy" {
            &mut self.bids
        } else {
            &mut self.asks
        };
        book.entry(order.price).or_default().push_back(order);
    }

    /// Remove stale or empty price levels
    pub fn prune_book(&mut self) {
        self.bids.retain(|_, q| !q.is_empty());
        self.asks.retain(|_, q| !q.is_empty());
    }

    /// Sync last matched price externally
    pub fn sync_last_price(&mut self, price: u64) {
        self.last_price = price;
    }

    /// Get last price for delta check
    pub fn get_last_price(&self) -> u64 {
        self.last_price
    }
}
