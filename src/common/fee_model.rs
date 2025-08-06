// ==========================================================
// fee_model.rs — Domex Fuel Burn & Fee Calculation Model
// ==========================================================
//
// This module defines the minimum fuel rules, vault-specific
// fee policies, and conversion logic for third-party delegation.
// Used by vault matchers, withdrawal processors, and validators.
//

use crate::token_config::*;
use crate::delegation::DelegationClaim;

/// Fee structure for a submitted proof (withdraw, trade, onboarding)
pub struct FuelFee {
    /// Amount of DOMEX burned by the user
    pub fuel_burned: u64,

    /// Optional delegator involved (if fuel was sponsored)
    pub delegator: Option<DelegationClaim>,
}

impl FuelFee {
    /// Check if the fuel meets the minimum required burn amount
    pub fn is_valid_fuel(&self) -> bool {
        self.fuel_burned >= MIN_PROOF_FUEL_BURN
    }

    /// Calculate delegator share, if sponsored
    pub fn delegator_cut(&self) -> u64 {
        match &self.delegator {
            Some(d) => d.requested_fee_domex,
            None => 0,
        }
    }

    /// Calculate net fuel that should be recycled
    pub fn recycled_fuel(&self) -> u64 {
        if self.fuel_burned < self.delegator_cut() {
            0
        } else {
            recycled_fuel_amount(self.fuel_burned - self.delegator_cut())
        }
    }

    /// True if fuel was provided by a delegator
    pub fn is_delegated(&self) -> bool {
        self.delegator.is_some()
    }
}
