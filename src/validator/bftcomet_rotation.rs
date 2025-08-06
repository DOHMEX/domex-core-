// ==========================================================
// src/validator/bftcomet_rotation.rs — Domex BFT-Comet Validator Rotation
// ==========================================================
//
// Handles validator committee rotation, attestation quorum checks,
// origin validator proof aggregation, and code integrity enforcement.
//
// Committee = 300 rotating validators + N selected origin validators (e.g., 1–50,000)
// All must submit matching attestations and identical code hashes.
// Used for quantum-safe ZK proof finality and validator slashing.
//

use std::collections::{HashSet, VecDeque, HashMap};
use crate::validator::attestation::ProofAttestation;
use crate::common::poseidon_utils::poseidon_hash_u64;
use chrono::Utc;

/// Constants
pub const MINORITY_SIZE: usize = 300;

/// Represents a validator in the rotation set
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Validator {
    pub id: String,               // Poseidon identity or pubkey
    pub last_active_epoch: u64,  // Last epoch the validator participated in
    pub stake: u64,              // Validator stake for ranking
    pub code_hash: String,       // Hash of validator bytecode (for slashing)
}

/// Committee rotation and validation logic
pub struct BFTCometRotation {
    minority_committee: VecDeque<Validator>, // Rotating 300 validators
    majority_validators: HashSet<Validator>, // From 100,000 pool
    origin_validators: HashSet<Validator>,   // Dynamic set (1–50,000) per epoch
    full_committee: HashSet<Validator>,      // 300 + origin
    epoch: u64,
    epoch_start_ts: u64,
}

impl BFTCometRotation {
    /// Initialize with selected validator sets and epoch
    pub fn new(
        minority: Vec<Validator>,
        majority: Vec<Validator>,
        origin: Vec<Validator>,
        epoch: u64,
    ) -> Self {
        let minority_committee = VecDeque::from(minority);
        let majority_validators = majority.into_iter().collect::<HashSet<_>>();
        let origin_validators = origin.into_iter().collect::<HashSet<_>>();

        let mut full_committee = minority_committee.iter().cloned().collect::<HashSet<_>>();
        full_committee.extend(origin_validators.iter().cloned());

        Self {
            minority_committee,
            majority_validators,
            origin_validators,
            full_committee,
            epoch,
            epoch_start_ts: Utc::now().timestamp() as u64,
        }
    }

    /// Rotate 1 minority validator to the back
    pub fn rotate_minority(&mut self) {
        if let Some(rotated) = self.minority_committee.pop_front() {
            self.minority_committee.push_back(rotated);
        }
    }

    /// Update full committee (minority + origin validators)
    pub fn update_full_committee(&mut self) {
        self.full_committee = self.minority_committee.iter().cloned().collect();
        self.full_committee.extend(self.origin_validators.iter().cloned());
    }

    /// Return current full committee
    pub fn current_committee(&self) -> &HashSet<Validator> {
        &self.full_committee
    }

    /// Validate all submitted attestations match in hash
    pub fn validate_attestations(&self, attestations: &[ProofAttestation]) -> bool {
        if attestations.len() != self.full_committee.len() {
            return false;
        }
        let hashes: HashSet<_> = attestations.iter().map(|a| &a.attestation_hash).collect();
        hashes.len() == 1
    }

    /// Check if all validators run identical code
    pub fn validate_code_hashes(&self) -> bool {
        let hashes: HashSet<_> = self.full_committee.iter().map(|v| &v.code_hash).collect();
        hashes.len() == 1
    }

    /// Detect validators submitting mismatched attestations
    pub fn detect_slashable_validators(&self, attestations: &[ProofAttestation]) -> Vec<String> {
        if attestations.len() != self.full_committee.len() {
            return vec![];
        }

        let mut counts: HashMap<&String, usize> = HashMap::new();
        for att in attestations {
            *counts.entry(&att.attestation_hash).or_insert(0) += 1;
        }

        let majority_hash = counts.iter().max_by_key(|(_, count)| *count).map(|(h, _)| (*h).clone());
        if majority_hash.is_none() {
            return vec![];
        }

        let mut offenders = vec![];
        for (att, validator) in attestations.iter().zip(self.full_committee.iter()) {
            if &att.attestation_hash != &majority_hash.clone().unwrap() {
                offenders.push(validator.id.clone());
            }
        }

        offenders
    }

    /// Advance epoch, rotate, and revalidate
    pub fn epoch_step(&mut self, attestations: &[ProofAttestation]) -> bool {
        self.epoch += 1;
        self.epoch_start_ts = Utc::now().timestamp() as u64;
        self.rotate_minority();
        self.update_full_committee();

        self.validate_attestations(attestations) && self.validate_code_hashes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_validator(id: &str, code_hash: &str) -> Validator {
        Validator {
            id: id.to_string(),
            last_active_epoch: 0,
            stake: 1000,
            code_hash: code_hash.to_string(),
        }
    }

    #[test]
    fn test_rotation_and_attestation_validation() {
        let minority = (0..MINORITY_SIZE)
            .map(|i| make_validator(&format!("minority{}", i), "code123"))
            .collect();

        let origin = (0..10)
            .map(|i| make_validator(&format!("origin{}", i), "code123"))
            .collect();

        let majority = (0..500)
            .map(|i| make_validator(&format!("majority{}", i), "code123"))
            .collect();

        let mut rotation = BFTCometRotation::new(minority, majority, origin, 1);

        let attestation = ProofAttestation {
            vault_id: "vaultx".into(),
            token: "dETH".into(),
            size: 1,
            owner_hash: "poseidon_user".into(),
            zk_root: "poseidon_root".into(),
            attestation_hash: "samehash".into(),
            timestamp: Utc::now().timestamp() as u64,
        };

        let full_size = rotation.full_committee.len();
        let attestations = vec![attestation.clone(); full_size];

        assert!(rotation.validate_attestations(&attestations));
        assert!(rotation.validate_code_hashes());
        assert!(rotation.epoch_step(&attestations));
    }
            }
