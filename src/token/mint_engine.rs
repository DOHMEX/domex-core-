// ==========================================================
// mint_engine.rs — DOMEX Validator Mint Logic
// ==========================================================
//
// Handles minting logic for DOMEX token:
// - Rewarding validators
// - Enforcing supply cap
// - Integrates with token_state.rs
//

use super::token_config::*;
use super::token_state::TokenState;

/// Mints reward tokens to validator and updates global token state.
///
/// Returns `Ok(())` if reward was successful, else `Err` with reason.
pub fn mint_validator_reward(
    state: &mut TokenState,
    validator_id: &str,
    num_blocks: u64,
) -> Result<u64, String> {
    let reward_amount = num_blocks * MAX_VALIDATOR_REWARD;

    if reward_amount == 0 {
        return Err("Zero reward amount".into());
    }

    // Check if pool has enough
    let available = state.available_reward_pool();
    if reward_amount > available {
        return Err("Insufficient reward pool available".into());
    }

    // Mint the reward
    state.mint_reward(reward_amount)?;

    // Log or broadcast event (placeholder)
    println!(
        "[MintEngine] ✅ Validator {} rewarded: {:.6} DOMEX ({} blocks)",
        validator_id,
        reward_amount as f64 / DOMEX_DECIMALS as f64,
        num_blocks
    );

    Ok(reward_amount)
}
