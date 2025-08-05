// src/zk_client/cold_deposit.rs

use crate::types::common::{Token, ZkIdentity, DepositAddress};
use serde::{Serialize, Deserialize};

/// Represents a deposit detected on a native chain,
/// but not yet claimed (i.e., not yet used to fill a vault).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColdDeposit {
    pub token: Token,
    pub amount: f64,
    pub deposit_address: DepositAddress,
    pub tx_hash: String,
    pub poseidon_identity: ZkIdentity,
    pub claimed: bool,
    pub autovault: bool,           // Indicates whether this deposit is eligible for proof-free vault onboarding
    pub timestamp: Option<u64>,    // Optional UNIX time (seconds) when the deposit was detected
    pub delegate_pubkey: Option<[u8; 32]>, // Optional delegate identity (used if autovault is true)
}

impl ColdDeposit {
    /// Creates a new unclaimed ColdDeposit record.
    ///
    /// Arguments:
    /// - `token`: Token being deposited (e.g., BTC, ETH)
    /// - `amount`: Amount of the token detected
    /// - `deposit_address`: Script-derived address the user sent funds to
    /// - `tx_hash`: The native chain transaction hash
    /// - `poseidon_identity`: Identity hash of the user (Poseidon(pubkey))
    /// - `autovault`: If true, vault can be auto-filled without ZK onboarding proof
    /// - `timestamp`: Optional UNIX timestamp of detection
    /// - `delegate_pubkey`: Optional public key of the authorized delegate (if applicable)
    pub fn new(
        token: Token,
        amount: f64,
        deposit_address: DepositAddress,
        tx_hash: String,
        poseidon_identity: ZkIdentity,
        autovault: bool,
        timestamp: Option<u64>,
        delegate_pubkey: Option<[u8; 32]>,
    ) -> Self {
        Self {
            token,
            amount,
            deposit_address,
            tx_hash,
            poseidon_identity,
            claimed: false,
            autovault,
            timestamp,
            delegate_pubkey,
        }
    }

    /// Marks this deposit as claimed (after successful onboarding or AutoVault issuance)
    pub fn mark_claimed(&mut self) {
        self.claimed = true;
    }

    /// Checks whether the deposit is eligible to be onboarded into a vault
    pub fn is_claimable(&self) -> bool {
        !self.claimed
    }

    /// Validates whether this deposit belongs to the expected identity/token pair
    pub fn matches_identity_token(&self, identity: &ZkIdentity, token: &Token) -> bool {
        &self.poseidon_identity == identity && &self.token == token
    }
}
