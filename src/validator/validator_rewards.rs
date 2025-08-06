// ==========================================================
// validator_rewards.rs — Domex Validator Reward + Slashing Engine
// ==========================================================
//
// Handles per-block reward distribution and slashing for the 301-core validator group.
// Uses constants from token_config.rs and validator stake context.
//
// Final state is recorded after successful proof attestation and quorum agreement.
//

use crate::token::token_config::{
    MAX_VALIDATOR_REWARD,
    recycled_fuel_amount,
    DOMEX_DECIMAL_MULTIPLIER,
};

use crate::validator::validator_identity::ValidatorId;
use std::collections::HashMap;

/// Validator reward types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RewardType {
    Core300,        // The 300 fixed validators per block
    SelectedMajority, // The 1 validator selected by majority
}

/// Validator reward info
#[derive(Debug, Clone)]
pub struct ValidatorReward {
    pub validator: ValidatorId,
    pub reward_type: RewardType,
    pub tx_count: u64,
}

/// Computes the reward per validator based on role and transaction count
pub fn compute_reward(tx_count: u64, role: RewardType) -> u64 {
    match role {
        RewardType::Core300 => {
            let reward = (tx_count / 10_000) * 10 * DOMEX_DECIMAL_MULTIPLIER;
            reward.min(MAX_VALIDATOR_REWARD)
        },
        RewardType::SelectedMajority => {
            6 * DOMEX_DECIMAL_MULTIPLIER
        },
    }
}

/// Generates reward map for current block based on validator activity
pub fn generate_block_rewards(
    attestations: &[ValidatorReward],
) -> HashMap<ValidatorId, u64> {
    let mut rewards = HashMap::new();

    for entry in attestations {
        let amount = compute_reward(entry.tx_count, entry.reward_type);
        rewards
            .entry(entry.validator.clone())
            .and_modify(|val| *val += amount)
            .or_insert(amount);
    }

    rewards
}

/// Calculates slash penalty (75% of stake)
pub fn slash_penalty(staked_amount: u64) -> u64 {
    (staked_amount * 75) / 100
}
