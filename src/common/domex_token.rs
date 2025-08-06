// ==========================================================
// domex_token.rs — Core DOMEX Token Logic (Mint, Burn, Supply)
// ==========================================================
//
// Handles DOMEX token balance updates, validator mint rewards, and fuel burn logic.
// Works alongside token_config.rs for constants and supply cap enforcement.

use std::collections::HashMap;
use crate::token_config::*;

pub struct DomexTokenLedger {
    balances: HashMap<String, u64>, // Maps address (or vault_id) → balance
    total_minted: u64,
    total_burned: u64,
}

impl DomexTokenLedger {
    /// Creates a new DOMEX token ledger with initial validator allocation
    pub fn new(first_validator_address: &str) -> Self {
        let mut balances = HashMap::new();
        balances.insert(first_validator_address.to_string(), FIRST_VALIDATOR_MINT);

        DomexTokenLedger {
            balances,
            total_minted: FIRST_VALIDATOR_MINT,
            total_burned: 0,
        }
    }

    /// Returns current balance of an address
    pub fn balance_of(&self, address: &str) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }

    /// Burns DOMEX tokens from an address (used as fuel)
    pub fn burn(&mut self, address: &str, amount: u64) -> Result<(), String> {
        let current_balance = self.balance_of(address);
        if amount > current_balance {
            return Err("Insufficient balance to burn".into());
        }

        self.balances.insert(address.to_string(), current_balance - amount);
        self.total_burned += amount;
        Ok(())
    }

    /// Mints new DOMEX to an address (used for validator rewards)
    pub fn mint(&mut self, address: &str, amount: u64) -> Result<(), String> {
        let new_total = self.total_minted + amount;
        if new_total > DOMEX_TOTAL_SUPPLY {
            return Err("Cannot mint beyond total supply cap".into());
        }

        let current_balance = self.balance_of(address);
        self.balances.insert(address.to_string(), current_balance + amount);
        self.total_minted = new_total;
        Ok(())
    }

    /// Recycles burned DOMEX back into unminted pool (for validator rewards)
    pub fn recycled_supply(&self) -> u64 {
        recycled_fuel_amount(self.total_burned)
    }

    /// Returns current mint/burn/supply stats
    pub fn stats(&self) -> (u64, u64, u64) {
        (
            self.total_minted,
            self.total_burned,
            DOMEX_TOTAL_SUPPLY - self.total_minted + self.recycled_supply(),
        )
    }
}
