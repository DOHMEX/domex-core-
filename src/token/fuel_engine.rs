// ==========================================================
// fuel_engine.rs — DOMEX Burn Recycling and Fuel Accounting
// ==========================================================
//
// Used in the token module to:
// - Track how much DOMEX was burned as fuel
// - Recycle fuel back into the unminted reward pool
// - Calculate validator-eligible pool balances
//

use super::token_config::{FUEL_RECYCLE_RATIO, recycled_fuel_amount, UNMINTED_REWARD_POOL};
use std::sync::atomic::{AtomicU64, Ordering};

/// Global atomic counter for tracking total fuel burned
static TOTAL_BURNED_FUEL: AtomicU64 = AtomicU64::new(0);

/// Global atomic counter for tracking total fuel recycled
static TOTAL_RECYCLED_FUEL: AtomicU64 = AtomicU64::new(0);

/// Called when a valid fuel burn is verified
pub fn process_fuel_burn(burn_amount: u64) {
    TOTAL_BURNED_FUEL.fetch_add(burn_amount, Ordering::SeqCst);
    let recycled = recycled_fuel_amount(burn_amount);
    TOTAL_RECYCLED_FUEL.fetch_add(recycled, Ordering::SeqCst);
}

/// Returns total amount of DOMEX burned as fuel
pub fn total_fuel_burned() -> u64 {
    TOTAL_BURNED_FUEL.load(Ordering::SeqCst)
}

/// Returns total amount of DOMEX recycled into unminted pool
pub fn total_fuel_recycled() -> u64 {
    TOTAL_RECYCLED_FUEL.load(Ordering::SeqCst)
}

/// Returns current balance of unminted DOMEX available for validator rewards
pub fn current_unminted_pool() -> u64 {
    UNMINTED_REWARD_POOL + total_fuel_recycled()
}
