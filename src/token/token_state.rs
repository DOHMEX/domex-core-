// ==========================================================
// token_state.rs — DOMEX Token Mint, Burn & Recycle State
// ==========================================================
//
// Handles internal accounting of:
// - Minted supply
// - Burned + recycled fuel
// - Reward pool management
// Ensures total supply cap is never exceeded.
//

use super::token_config::*;

#[derive(Debug, Clone)]
pub struct TokenState {
    /// Total amount of DOMEX minted so far
    pub minted: u64,

    /// Total amount of DOMEX burned (e.g., fuel usage)
    pub burned: u64,

    /// Total amount recycled into the reward pool
    pub recycled: u64,
}

impl TokenState {
    /// Creates a new TokenState with only the genesis validator mint applied
    pub fn new() -> Self {
        Self {
            minted: FIRST_VALIDATOR_MINT,
            burned: 0,
            recycled: 0,
        }
    }

    /// Mint new DOMEX tokens for validator reward
    pub fn mint_reward(&mut self, amount: u64) -> Result<(), String> {
        if self.minted + amount > DOMEX_TOTAL_SUPPLY {
            return Err("Minting exceeds total DOMEX supply cap".into());
        }

        self.minted += amount;
        Ok(())
    }

    /// Record a fuel burn event (e.g., from proof submission)
    pub fn record_burn(&mut self, amount: u64) {
        self.burned += amount;

        let recycled = recycled_fuel_amount(amount);
        self.recycled += recycled;
    }

    /// Get available reward pool (recycled + unminted)
    pub fn available_reward_pool(&self) -> u64 {
        let total_allowed = UNMINTED_REWARD_POOL;
        let already_distributed = self.minted - FIRST_VALIDATOR_MINT;
        let remaining = total_allowed.saturating_sub(already_distributed);
        remaining + self.recycled
    }

    /// Human-readable status snapshot
    pub fn summary(&self) -> String {
        format!(
            "Minted: {:.6}, Burned: {:.6}, Recycled: {:.6}, RewardPool: {:.6}",
            self.minted as f64 / DOMEX_DECIMALS as f64,
            self.burned as f64 / DOMEX_DECIMALS as f64,
            self.recycled as f64 / DOMEX_DECIMALS as f64,
            self.available_reward_pool() as f64 / DOMEX_DECIMALS as f64,
        )
    }
}
