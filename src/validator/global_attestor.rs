// src/validator/global_attestor.rs

use std::collections::{HashMap, HashSet};
use chrono::Utc;
use crate::validator::attestation::ProofAttestation;
use crate::validator::slashing_engine::SlashingEngine;
use crate::types::bftcomet::Validator;
use crate::common::poseidon_utils::poseidon_hash_many;

/// Represents an aggregated ZK batch submitted by one validator
#[derive(Debug, Clone)]
pub struct ProofSubmission {
    pub validator_id: String,
    pub batch_hash: String,
    pub attestations: Vec<ProofAttestation>,
    pub timestamp: u64,
}

/// GlobalAttestor handles aggregation, verification, and slashing
pub struct GlobalAttestor {
    pub minority_committee: HashSet<String>,               // 300 fixed
    pub submissions: Vec<ProofSubmission>,                 // All incoming validator proofs
    pub slashing_engine: SlashingEngine,                   // Handles validator slashing
    pub verified_attestation_hash: Option<String>,         // The final agreed proof hash
}

impl GlobalAttestor {
    pub fn new(minority_committee: HashSet<String>, slashing_engine: SlashingEngine) -> Self {
        Self {
            minority_committee,
            submissions: Vec::new(),
            slashing_engine,
            verified_attestation_hash: None,
        }
    }

    /// Accept proof from global validator (up to 99,700 origin)
    pub fn submit_proof(&mut self, submission: ProofSubmission) {
        self.submissions.push(submission);
    }

    /// Run attestation by minority (300)
    pub fn run_attestation_round(&mut self) -> bool {
        if self.submissions.is_empty() {
            return false;
        }

        // Step 1: Aggregate all hashes submitted
        let mut hash_count: HashMap<String, usize> = HashMap::new();

        for sub in &self.submissions {
            *hash_count.entry(sub.batch_hash.clone()).or_insert(0) += 1;
        }

        // Step 2: Find most common hash (quorum)
        if let Some((most_common_hash, _)) = hash_count.iter().max_by_key(|(_, v)| *v) {
            self.verified_attestation_hash = Some(most_common_hash.clone());

            // Step 3: Identify and slash invalid proof submissions
            for sub in &self.submissions {
                if &sub.batch_hash != most_common_hash {
                    let dummy_attestation = sub.attestations.get(0).cloned().unwrap_or_else(|| ProofAttestation {
                        vault_id: "unknown".into(),
                        token: "dUNKNOWN".into(),
                        size: 0,
                        owner_hash: "0".into(),
                        zk_root: "0".into(),
                        attestation_hash: sub.batch_hash.clone(),
                        timestamp: sub.timestamp,
                    });

                    self.slashing_engine.slash_for_attestation_mismatch(
                        &sub.validator_id,
                        &dummy_attestation,
                        most_common_hash,
                        0,
                    );
                }
            }

            return true;
        }

        false
    }

    /// Poseidon hash of all submission roots (for broadcasting)
    pub fn generate_proof_attest_root(&self) -> Option<String> {
        if self.verified_attestation_hash.is_none() {
            return None;
        }

        let roots: Vec<String> = self
            .submissions
            .iter()
            .map(|s| s.batch_hash.clone())
            .collect();

        Some(poseidon_hash_many(&roots))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validator::slashing_engine::SlashingEngine;
    use crate::validator::validator_registry::ValidatorRegistry;

    fn mock_attestation(hash: &str) -> ProofAttestation {
        ProofAttestation {
            vault_id: "vault123".into(),
            token: "dBTC".into(),
            size: 10,
            owner_hash: "poseidon(owner)".into(),
            zk_root: "poseidon(root)".into(),
            attestation_hash: hash.into(),
            timestamp: Utc::now().timestamp() as u64,
        }
    }

    #[test]
    fn test_attestation_round() {
        let mut registry = ValidatorRegistry::new();
        registry.add_validator(Validator {
            id: "v1".into(),
            stake: 1000,
            last_active_epoch: 1,
        });

        let engine = SlashingEngine::new(registry);
        let mut attestor = GlobalAttestor::new(HashSet::new(), engine);

        attestor.submit_proof(ProofSubmission {
            validator_id: "v1".into(),
            batch_hash: "hash_a".into(),
            attestations: vec![mock_attestation("hash_a")],
            timestamp: Utc::now().timestamp() as u64,
        });

        attestor.submit_proof(ProofSubmission {
            validator_id: "v2".into(),
            batch_hash: "hash_b".into(),
            attestations: vec![mock_attestation("hash_b")],
            timestamp: Utc::now().timestamp() as u64,
        });

        let success = attestor.run_attestation_round();
        assert!(success);
        assert_eq!(attestor.verified_attestation_hash.is_some(), true);
    }
}
