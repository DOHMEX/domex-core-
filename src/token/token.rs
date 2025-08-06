// ==========================================================
// token.rs — DOMEX Token Controller
// ==========================================================
//
// Core interface for DOMEX token operations:
// - Minting to validators
// - Fuel burn accounting
// - Supply cap enforcement
//

use super::token_config::*;
use super::token_state::TokenState;
use super::mint_engine::mint_validator_reward;

#[derive(Debug)]
pub struct DomexToken {
    pub state: TokenState,
}

impl DomexToken {
    /// Initializes a new DOMEX token context (fresh or restored state)
    pub fn new() -> Self {
        DomexToken {
            state: TokenState::new(),
        }
    }

    /// Mint reward for a validator over N blocks
    pub fn reward_validator(&mut self, validator_id: &str, blocks: u64) -> Result<u64, String> {
        mint_validator_reward(&mut self.state, validator_id, blocks)
    }

    /// Record fuel burn and return recycled amount
    pub fn burn_fuel(&mut self, burned: u64) -> u64 {
        let recycled = recycled_fuel_amount(burned);
        self.state.recycle(recycled);
        recycled
    }

    /// Returns total minted DOMEX so far
    pub fn total_minted(&self) -> u64 {
        self.state.total_minted
    }

    /// Returns available unminted supply
    pub fn available_pool(&self) -> u64 {
        self.state.available_reward_pool()
    }

    /// Returns supply cap
    pub fn supply_cap(&self) -> u64 {
        DOMEX_TOTAL_SUPPLY
    }
}
