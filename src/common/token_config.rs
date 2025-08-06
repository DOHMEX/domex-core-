// ==========================================================
// token_config.rs — Domex Token Constants and Burn Recycling Logic
// ==========================================================
//
// Defines DOMEX tokenomics: supply cap, validator mint, burn/recycle flow.
// Used by the reward engine, fuel verifier, and global validator system.
//

/// Base decimals for DOMEX token (1e6 for 6 decimals: like USDC)
pub const DOMEX_DECIMAL_MULTIPLIER: u64 = 1_000_000;

/// Total hard-capped DOMEX supply: 1 billion tokens (fixed forever)
pub const DOMEX_TOTAL_SUPPLY: u64 = 1_000_000_000 * DOMEX_DECIMAL_MULTIPLIER;

/// Genesis mint allocation to first validator (20%)
pub const FIRST_VALIDATOR_MINT: u64 = 200_000_000 * DOMEX_DECIMAL_MULTIPLIER;

/// Remaining unminted pool (used for validator rewards + fuel recycling)
pub const UNMINTED_REWARD_POOL: u64 = DOMEX_TOTAL_SUPPLY - FIRST_VALIDATOR_MINT;

/// Minimum fuel burn required for valid proof (can be scaled per vault)
pub const MIN_PROOF_FUEL_BURN: u64 = 10 * DOMEX_DECIMAL_MULTIPLIER / 1_000_000; // 0.00001 DOMEX

/// Maximum reward per validator per block (e.g., 2 DOMEX)
pub const MAX_VALIDATOR_REWARD: u64 = 2 * DOMEX_DECIMAL_MULTIPLIER;

/// Share of burned fuel recycled into the unminted pool (100%)
pub const FUEL_RECYCLE_RATIO: f64 = 1.0;

/// Calculates how much of a burned fuel amount gets recycled into the pool
pub fn recycled_fuel_amount(burned: u64) -> u64 {
    (burned as f64 * FUEL_RECYCLE_RATIO) as u64
}

/// Calculates how many validator reward tokens can be distributed from unminted pool
pub fn validator_reward_budget() -> u64 {
    UNMINTED_REWARD_POOL
}

/// Returns total DOMEX supply in decimal units
pub fn total_domex_human_readable() -> f64 {
    DOMEX_TOTAL_SUPPLY as f64 / DOMEX_DECIMAL_MULTIPLIER as f64
}
