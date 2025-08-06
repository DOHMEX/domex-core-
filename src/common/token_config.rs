// ==========================================================
// token_config.rs — Domex Token Constants and Burn Recycling Logic
// ==========================================================
//
// Defines DOMEX tokenomics: total supply, validator mint, fuel burn logic,
// and reward recycling system. Used by the reward engine, fuel validator,
// and delegation fee contracts.
//

/// Base decimals for DOMEX token (1e6 = 6 decimals like USDC)
pub const DOMEX_DECIMAL_MULTIPLIER: u64 = 1_000_000;

/// Hard-capped total supply of DOMEX: 1 billion tokens (fixed)
pub const DOMEX_TOTAL_SUPPLY: u64 = 1_000_000_000 * DOMEX_DECIMAL_MULTIPLIER;

/// Allocation minted at genesis to the first validator (20%)
pub const FIRST_VALIDATOR_MINT: u64 = 200_000_000 * DOMEX_DECIMAL_MULTIPLIER;

/// Unminted reward pool for future validator emissions + fuel burn recycling
pub const UNMINTED_REWARD_POOL: u64 = DOMEX_TOTAL_SUPPLY - FIRST_VALIDATOR_MINT;

/// Minimum fuel burn required for proof submission (scaled per vault)
pub const MIN_PROOF_FUEL_BURN: u64 = 10 * DOMEX_DECIMAL_MULTIPLIER / 1_000_000; // 0.00001 DOMEX

/// Maximum DOMEX reward per validator per attested block
pub const MAX_VALIDATOR_REWARD: u64 = 2 * DOMEX_DECIMAL_MULTIPLIER;

/// Proportion of burned DOMEX recycled back into unminted pool (e.g. 100%)
pub const FUEL_RECYCLE_RATIO: f64 = 1.0;

/// Calculates how much of the burned fuel gets recycled into the global pool
pub fn recycled_fuel_amount(burned: u64) -> u64 {
    (burned as f64 * FUEL_RECYCLE_RATIO) as u64
}

/// Validator reward budget per epoch or block — drawn from unminted pool
pub fn validator_reward_budget() -> u64 {
    UNMINTED_REWARD_POOL
}

/// Returns total DOMEX supply in human-readable decimal (e.g. 1_000_000_000.0)
pub fn total_domex_human_readable() -> f64 {
    DOMEX_TOTAL_SUPPLY as f64 / DOMEX_DECIMAL_MULTIPLIER as f64
}

/// Calculates user-visible decimal value from raw token amount
pub fn to_domex_decimal(units: u64) -> f64 {
    units as f64 / DOMEX_DECIMAL_MULTIPLIER as f64
}

/// Converts decimal value back into on-chain integer representation
pub fn from_domex_decimal(amount: f64) -> u64 {
    (amount * DOMEX_DECIMAL_MULTIPLIER as f64).round() as u64
}
