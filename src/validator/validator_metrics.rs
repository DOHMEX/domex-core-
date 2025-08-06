// src/validator/validator_metrics.rs

use std::collections::HashMap;
use crate::types::common::{ValidatorId, VaultId};
use crate::types::proof::ProofHash;
use chrono::{Utc, DateTime};

/// Individual validator activity snapshot
#[derive(Debug, Clone)]
pub struct ValidatorMetrics {
    pub validator_id: ValidatorId,
    pub zk_proofs_verified: u64,
    pub vault_attestations: u64,
    pub fuel_burn_matched: u64,
    pub total_latency_ms: u128,
    pub last_attested: Option<DateTime<Utc>>,
    pub slashed: bool,

    // Added fields from second version
    pub valid_proofs: u64,
    pub invalid_proofs: u64,
    pub committee_participations: u64,
    pub origin_submissions: u64,
    pub last_epoch_active: u64,
}

impl ValidatorMetrics {
    pub fn new(id: ValidatorId) -> Self {
        Self {
            validator_id: id,
            zk_proofs_verified: 0,
            vault_attestations: 0,
            fuel_burn_matched: 0,
            total_latency_ms: 0,
            last_attested: None,
            slashed: false,
            valid_proofs: 0,
            invalid_proofs: 0,
            committee_participations: 0,
            origin_submissions: 0,
            last_epoch_active: 0,
        }
    }

    /// Record proof submission with latency and validity
    pub fn record_proof(&mut self, is_valid: bool, latency_ms: u128, epoch: u64) {
        if is_valid {
            self.valid_proofs += 1;
        } else {
            self.invalid_proofs += 1;
        }

        self.zk_proofs_verified += 1;
        self.total_latency_ms += latency_ms;
        self.last_attested = Some(Utc::now());
        self.last_epoch_active = epoch;
    }

    /// Record vault attestation
    pub fn record_vault_attestation(&mut self, epoch: u64) {
        self.vault_attestations += 1;
        self.last_epoch_active = epoch;
    }

    /// Record delegator fuel-match success
    pub fn record_fuel_match(&mut self) {
        self.fuel_burn_matched += 1;
    }

    /// Record selection in 300-attestor committee
    pub fn record_committee_participation(&mut self, epoch: u64) {
        self.committee_participations += 1;
        self.last_epoch_active = epoch;
    }

    /// Record being origin proof submitter
    pub fn record_origin_submission(&mut self, epoch: u64) {
        self.origin_submissions += 1;
        self.last_epoch_active = epoch;
    }

    /// Slash validator
    pub fn slash(&mut self) {
        self.slashed = true;
    }

    /// Get average latency per proof
    pub fn average_latency(&self) -> Option<f64> {
        if self.zk_proofs_verified == 0 {
            None
        } else {
            Some(self.total_latency_ms as f64 / self.zk_proofs_verified as f64)
        }
    }

    /// Check if validator is still active
    pub fn is_active(&self) -> bool {
        !self.slashed && self.last_attested.is_some()
    }
}

/// Central metrics registry for all validators
#[derive(Debug)]
pub struct ValidatorMetricsRegistry {
    pub all: HashMap<ValidatorId, ValidatorMetrics>,
}

impl ValidatorMetricsRegistry {
    pub fn new() -> Self {
        Self {
            all: HashMap::new(),
        }
    }

    /// Get or create validator metrics entry
    pub fn get_or_create(&mut self, id: ValidatorId) -> &mut ValidatorMetrics {
        self.all.entry(id.clone()).or_insert_with(|| ValidatorMetrics::new(id))
    }

    /// Get read-only metrics
    pub fn get(&self, validator_id: &ValidatorId) -> Option<&ValidatorMetrics> {
        self.all.get(validator_id)
    }

    /// Top validators by number of total proofs submitted
    pub fn top_validators_by_proof(&self, count: usize) -> Vec<&ValidatorMetrics> {
        let mut list: Vec<&ValidatorMetrics> = self.all.values().collect();
        list.sort_by_key(|v| -(v.zk_proofs_verified as i64));
        list.into_iter().take(count).collect()
    }

    /// Get all validators marked as slashed
    pub fn get_slashed(&self) -> Vec<&ValidatorMetrics> {
        self.all.values().filter(|v| v.slashed).collect()
    }
        }
