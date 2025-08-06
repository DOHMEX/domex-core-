// ==========================================================
// staking_pool.rs — Domex Validator Staking Pool Logic
// ==========================================================
//
// Handles staking deposits, withdrawals, slashing, and locked status.
// Used during validator onboarding and punishment phases.
//
// Delegation logic is handled separately in delegation_pool.rs.
//

use std::collections::HashMap;
use crate::validator::validator_identity::ValidatorId;
use crate::token::token_config::DOMEX_DECIMAL_MULTIPLIER;

pub type Amount = u64;

/// Validator staking state
#[derive(Debug, Clone)]
pub struct StakeInfo {
    pub staked_amount: Amount,
    pub is_locked: bool, // true if under slash review or proof mismatch
}

/// Global staking pool
#[derive(Default)]
pub struct StakingPool {
    pool: HashMap<ValidatorId, StakeInfo>,
}

impl StakingPool {
    /// Create a new empty staking pool
    pub fn new() -> Self {
        Self {
            pool: HashMap::new(),
        }
    }

    /// Deposit stake for validator
    pub fn deposit(&mut self, validator: ValidatorId, amount: Amount) {
        let entry = self.pool.entry(validator).or_insert(StakeInfo {
            staked_amount: 0,
            is_locked: false,
        });

        entry.staked_amount += amount;
    }

    /// Slash validator for bad proof (75%)
    pub fn slash(&mut self, validator: &ValidatorId) -> Option<Amount> {
        if let Some(info) = self.pool.get_mut(validator) {
            let penalty = (info.staked_amount * 75) / 100;
            info.staked_amount -= penalty;
            info.is_locked = true;
            Some(penalty)
        } else {
            None
        }
    }

    /// Unlock validator after review
    pub fn unlock(&mut self, validator: &ValidatorId) {
        if let Some(info) = self.pool.get_mut(validator) {
            info.is_locked = false;
        }
    }

    /// Check available balance
    pub fn get_balance(&self, validator: &ValidatorId) -> Amount {
        self.pool
            .get(validator)
            .map(|info| info.staked_amount)
            .unwrap_or(0)
    }

    /// Withdraw stake (only if not locked)
    pub fn withdraw(&mut self, validator: &ValidatorId, amount: Amount) -> Result<(), String> {
        if let Some(info) = self.pool.get_mut(validator) {
            if info.is_locked {
                return Err("Stake is locked due to slashing or review".into());
            }
            if info.staked_amount < amount {
                return Err("Insufficient stake".into());
            }
            info.staked_amount -= amount;
            Ok(())
        } else {
            Err("Validator not found".into())
        }
    }

    /// Minimum stake required to join (example: 10,000 DOMEX)
    pub fn minimum_required_stake() -> Amount {
        10_000 * DOMEX_DECIMAL_MULTIPLIER
    }
}
