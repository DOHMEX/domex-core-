// ============================================  
// types/orderbook.rs — Shared OrderBook Types  
// ============================================  

use std::collections::HashMap;

/// Alias for Poseidon-based identity hash  
pub type PoseidonHash = [u8; 32];

/// Order intent type (buy or sell)  
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderIntent {
    Buy,
    Sell,
}

/// A single order instruction submitted by a user  
#[derive(Debug, Clone)]
pub struct OrderInstruction {
    pub vault_id: String,
    pub token: String,
    pub intent: OrderIntent,
    pub size: u64,
    pub price: u64,
    pub owner_hash: PoseidonHash,
    pub counterparty_hash: PoseidonHash, // Filled by matching engine
}

/// Result of a completed and verified trade  
#[derive(Debug, Clone)]
pub struct TradeResult {
    pub vault_id: String,
    pub executed_price: u64,
    pub buyer: PoseidonHash,
    pub seller: PoseidonHash,
    pub token: String,
    pub size: u64,
    pub balance_delta: Vec<BalanceChange>,
}

/// Describes a single balance change (used in Merkle + ZK proof)  
#[derive(Debug, Clone)]
pub struct BalanceChange {
    pub identity: PoseidonHash,
    pub token: String,
    pub delta: i64, // +ve for credit, -ve for debit
}

/// VaultState tracks per-identity token balances and vault ID  
#[derive(Debug)]
pub struct VaultState {
    pub vault_id: String,
    pub balances: HashMap<(PoseidonHash, String), u64>, // (identity, token) → balance
}

impl VaultState {
    pub fn get_balance(&self, identity: &PoseidonHash, token: &str) -> u64 {
        *self.balances.get(&(identity.clone(), token.to_string())).unwrap_or(&0)
    }

    pub fn decrease_balance(&mut self, identity: &PoseidonHash, token: &str, amount: u64) {
        let key = (identity.clone(), token.to_string());
        let entry = self.balances.entry(key).or_insert(0);
        *entry = entry.saturating_sub(amount);
    }

    pub fn increase_balance(&mut self, identity: &PoseidonHash, token: &str, amount: u64) {
        let key = (identity.clone(), token.to_string());
        let entry = self.balances.entry(key).or_insert(0);
        *entry += amount;
    }
}

/// Proposal to be committed via Raft consensus  
#[derive(Debug, Clone)]
pub struct RaftProposal {
    pub vault_id: String,
    pub trade: TradeResult,
}

/// ZK onboarding proof submitted by a user to activate Phase 2
#[derive(Debug, Clone)]
pub struct ZkOnboardingProof {
    pub vault_id: String,
    pub owner_hash: PoseidonHash,
    pub merkle_root: String,     // From global validator
    pub proof_blob: Vec<u8>,     // Serialized ZK proof (e.g., Groth16, PlonK)
}

/// Internal activation registry entry
#[derive(Debug, Clone)]
pub struct VaultActivation {
    pub vault_id: String,
    pub owner_hash: PoseidonHash,
    pub zk_proof: ZkOnboardingProof,
    pub is_active: bool,
    }
