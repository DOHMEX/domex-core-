// ==========================================================
// token_config.rs — Domex Token Constants for Validator SDK
// ==========================================================
//
// Defines DOMEX validator rewards, token supply cap, burn recycling,
// and reward budgeting for attestation-based consensus.
//
// This file is scoped ONLY for validator-side logic (no user delegation).
//

/// Number of decimals for DOMEX token (1e6 = 6 decimals, like USDC)
pub const DOMEX_DECIMALS: u64 = 1_000_000;

/// Total fixed DOMEX supply: 1 billion tokens (non-inflationary)
pub const DOMEX_TOTAL_SUPPLY: u64 = 1_000_000_000 * DOMEX_DECIMALS;

/// Initial mint allocation to first validator (20%)
pub const FIRST_VALIDATOR_MINT: u64 = 200_000_000 * DOMEX_DECIMALS;

/// Remaining unminted pool for validator rewards and recycled fuel
pub const UNMINTED_POOL: u64 = DOMEX_TOTAL_SUPPLY - FIRST_VALIDATOR_MINT;


/// ==========================
/// Validator Reward Constants
/// ==========================

/// Reward for 1 validator selected by global majority (per valid proof)
pub const MAJORITY_SELECTED_VALIDATOR_REWARD: u64 = 6 * DOMEX_DECIMALS;

/// Reward for each of the 300 attestation validators (per 10,000 txs)
pub const CORE_VALIDATOR_REWARD_PER_10K_TX: u64 = 10 * DOMEX_DECIMALS;

/// Max number of such parallel proofs in a single block (for budgeting)
pub const MAX_PARALLEL_PROOFS: u64 = 40_000;

/// Total validator reward cap per block (computed from above)
pub fn max_block_reward_budget() -> u64 {
    (MAJORITY_SELECTED_VALIDATOR_REWARD * MAX_PARALLEL_PROOFS)
        + (CORE_VALIDATOR_REWARD_PER_10K_TX * 300)
}


/// ==========================
/// Fuel Burn and Recycling
/// ==========================

/// Minimum fuel burn required per ZK proof (0.00001 DOMEX)
pub const MIN_PROOF_FUEL_BURN: u64 = 10 * DOMEX_DECIMALS / 1_000_000;

/// Share of burned fuel recycled into reward pool (100%)
pub const FUEL_RECYCLE_RATIO: f64 = 1.0;

/// Recycles burned fuel back into unminted pool
pub fn recycled_fuel(burned: u64) -> u64 {
    (burned as f64 * FUEL_RECYCLE_RATIO) as u64
}


/// ==========================
/// DOMEX Supply Utilities
/// ==========================

/// Total supply in decimal units (e.g. 1,000,000,000.000000)
pub fn total_domex_human() -> f64 {
    DOMEX_TOTAL_SUPPLY as f64 / DOMEX_DECIMALS as f64
}

/// Returns how much of the DOMEX supply remains for rewards
pub fn remaining_reward_pool() -> u64 {
    UNMINTED_POOL
}
