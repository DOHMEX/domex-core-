// src/validator/slashing_engine.rs

use std::collections::HashMap;
use chrono::Utc;
use crate::types::bftcomet::Validator;
use crate::validator::validator_registry::ValidatorRegistry;
use crate::token::fuel_engine::burn_fuel_from_stake;
use crate::common::zk_utils::verify_attestation_hash;
use crate::validator::attestation::ProofAttestation;

/// Slashing event record for auditing
#[derive(Debug, Clone)]
pub struct SlashingEvent {
    pub validator_id: String,
    pub reason: String,
    pub slash_amount: u64,
    pub epoch: u64,
    pub timestamp: u64,
}

/// Main Domex slashing engine
pub struct SlashingEngine {
    pub registry: ValidatorRegistry,
    pub events: Vec<SlashingEvent>,
}

impl SlashingEngine {
    /// Initialize with validator registry
    pub fn new(registry: ValidatorRegistry) -> Self {
        Self {
            registry,
            events: Vec::new(),
        }
    }

    /// Slash validator if they submit a bad or mismatched attestation
    pub fn slash_for_attestation_mismatch(
        &mut self,
        validator_id: &str,
        submitted: &ProofAttestation,
        expected_hash: &str,
        epoch: u64,
    ) -> bool {
        // Step 1: Validate mismatch
        if submitted.attestation_hash != expected_hash {
            // Step 2: Calculate slash amount (e.g. 50% of stake)
            if let Some(v) = self.registry.get_validator(validator_id) {
                let slash_amount = v.stake / 2;

                // Step 3: Burn stake or fuel equivalent
                burn_fuel_from_stake(validator_id, slash_amount);

                // Step 4: Remove validator from registry
                self.registry.remove_validator(validator_id);

                // Step 5: Record event
                self.events.push(SlashingEvent {
                    validator_id: validator_id.to_string(),
                    reason: "Mismatched attestation hash".into(),
                    slash_amount,
                    epoch,
                    timestamp: Utc::now().timestamp() as u64,
                });

                return true;
            }
        }

        false
    }

    /// Returns all slash records for monitoring/auditing
    pub fn all_events(&self) -> &[SlashingEvent] {
        &self.events
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::bftcomet::Validator;

    fn mock_validator(id: &str, stake: u64) -> Validator {
        Validator {
            id: id.to_string(),
            stake,
            last_active_epoch: 0,
        }
    }

    #[test]
    fn test_slashing_logic() {
        let mut registry = ValidatorRegistry::new();
        let v = mock_validator("validator123", 1000);
        registry.add_validator(v.clone());

        let mut engine = SlashingEngine::new(registry);

        let attestation = ProofAttestation {
            vault_id: "vaultA".into(),
            token: "dBTC".into(),
            size: 5,
            owner_hash: "ownerPoseidon".into(),
            zk_root: "zkRootPoseidon".into(),
            attestation_hash: "badHash".into(),
            timestamp: Utc::now().timestamp() as u64,
        };

        let result = engine.slash_for_attestation_mismatch("validator123", &attestation, "expectedHash", 1);
        assert!(result);
        assert_eq!(engine.events.len(), 1);
    }
}
