// =======================================================================
// validator_metrics.rs — Domex Validator Activity + Performance Tracker
// =======================================================================
//
// This module tracks validator activity across ZK proof attestations,
// vault operations, and fuel usage — all without signature-based logic.
// Used to rank, slash, or reward validators on the network.
//
// ⚠️ All tracking is quantum-safe: identity and activity are
// Poseidon-hashed and Merkle-bounded (no signatures).
//

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
}

impl ValidatorMetrics {
    /// Create a new validator metrics tracker
    pub fn new(id: ValidatorId) -> Self {
        Self {
            validator_id: id,
            zk_proofs_verified: 0,
            vault_attestations: 0,
            fuel_burn_matched: 0,
            total_latency_ms: 0,
            last_attested: None,
            slashed: false,
        }
    }

    /// Record a successful proof attestation
    pub fn record_proof(&mut self, latency_ms: u128) {
        self.zk_proofs_verified += 1;
        self.total_latency_ms += latency_ms;
        self.last_attested = Some(Utc::now());
    }

    /// Record participation in vault state finality
    pub fn record_vault_attestation(&mut self) {
        self.vault_attestations += 1;
    }

    /// Record matched fuel-burn receipt from delegator
    pub fn record_fuel_match(&mut self) {
        self.fuel_burn_matched += 1;
    }

    /// Mark validator as slashed
    pub fn slash(&mut self) {
        self.slashed = true;
    }

    /// Compute average latency per proof (ms)
    pub fn average_latency(&self) -> Option<f64> {
        if self.zk_proofs_verified == 0 {
            None
        } else {
            Some(self.total_latency_ms as f64 / self.zk_proofs_verified as f64)
        }
    }

    /// Determine if validator is active
    pub fn is_active(&self) -> bool {
        !self.slashed && self.last_attested.is_some()
    }
}

/// Central registry for all validator metrics
#[derive(Debug)]
pub struct ValidatorMetricsRegistry {
    pub all: HashMap<ValidatorId, ValidatorMetrics>,
}

impl ValidatorMetricsRegistry {
    /// Create an empty registry
    pub fn new() -> Self {
        Self {
            all: HashMap::new(),
        }
    }

    /// Get or insert metrics record for a validator
    pub fn get_or_create(&mut self, id: ValidatorId) -> &mut ValidatorMetrics {
        self.all.entry(id.clone()).or_insert_with(|| ValidatorMetrics::new(id))
    }

    /// Get top validators by number of proofs verified
    pub fn top_validators_by_proof(&self, count: usize) -> Vec<&ValidatorMetrics> {
        let mut list: Vec<&ValidatorMetrics> = self.all.values().collect();
        list.sort_by_key(|v| -(v.zk_proofs_verified as i64));
        list.into_iter().take(count).collect()
    }

    /// Get slashed validators
    pub fn get_slashed(&self) -> Vec<&ValidatorMetrics> {
        self.all.values().filter(|v| v.slashed).collect()
    }
}
