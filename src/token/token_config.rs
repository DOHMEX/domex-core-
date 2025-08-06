// ==========================================================
// token_config.rs — DOMEX Tokenomics & Validator Reward Logic
// ==========================================================
//
// Defines core constants for:
// - DOMEX total supply and decimals
// - Validator minting rules
// - Fuel burn thresholds and recycling into reward pool
//

/// DOMEX token uses 6 decimal places (1e6), similar to USDC
pub const DOMEX_DECIMALS: u64 = 1_000_000;

/// Total capped supply: 1,000,000,000 DOMEX (fixed forever)
pub const DOMEX_TOTAL_SUPPLY: u64 = 1_000_000_000 * DOMEX_DECIMALS;

/// Initial validator mint (genesis validator gets 20%)
pub const FIRST_VALIDATOR_MINT: u64 = 200_000_000 * DOMEX_DECIMALS;

/// Remaining unminted supply used for validator rewards
pub const UNMINTED_REWARD_POOL: u64 = DOMEX_TOTAL_SUPPLY - FIRST_VALIDATOR_MINT;

/// Minimum fuel burn required to submit a valid ZK proof (e.g., 0.00001 DOMEX)
pub const MIN_PROOF_FUEL_BURN: u64 = 10 * DOMEX_DECIMALS / 1_000_000;

/// Maximum reward a validator can earn per proof round (e.g., 2 DOMEX)
pub const MAX_VALIDATOR_REWARD: u64 = 2 * DOMEX_DECIMALS;

/// Share of burned fuel that is recycled back into unminted pool (100%)
pub const FUEL_RECYCLE_RATIO: f64 = 1.0;

/// Calculate the amount of recycled DOMEX from fuel burned
pub fn recycled_fuel_amount(burned: u64) -> u64 {
    (burned as f64 * FUEL_RECYCLE_RATIO) as u64
}

/// View total supply in human-readable units (e.g., 1,000,000,000.0)
pub fn total_domex_human_readable() -> f64 {
    DOMEX_TOTAL_SUPPLY as f64 / DOMEX_DECIMALS as f64
}
