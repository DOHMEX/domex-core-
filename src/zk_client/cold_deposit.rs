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
}

impl ColdDeposit {
    /// Creates a new unclaimed ColdDeposit record
    pub fn new(
        token: Token,
        amount: f64,
        deposit_address: DepositAddress,
        tx_hash: String,
        poseidon_identity: ZkIdentity,
    ) -> Self {
        Self {
            token,
            amount,
            deposit_address,
            tx_hash,
            poseidon_identity,
            claimed: false,
        }
    }

    /// Marks this deposit as claimed (after successful onboarding proof)
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
