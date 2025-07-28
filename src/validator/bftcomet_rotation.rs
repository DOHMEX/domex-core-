// src/validator/bftcomet_rotation.rs

use std::collections::{HashSet, VecDeque};
use crate::validator::attestation::ProofAttestation;
use crate::utils::poseidon_hash;
use crate::zk_verifier::verify_zk_proof;
use chrono::Utc;

/// Constants
pub const MINORITY_SIZE: usize = 300;
pub const FULL_COMMITTEE_SIZE: usize = 301;

/// Represents a validator in the rotation set
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Validator {
    pub id: String,               // Poseidon identity or pubkey
    pub last_active_epoch: u64,   // Last epoch the validator participated in
    pub stake: u64,               // Validator stake for ranking
}

/// Committee rotation manager
pub struct BFTCometRotation {
    minority_committee: VecDeque<Validator>, // Rotating 300 minority validators
    majority_validators: HashSet<Validator>, // Validators outside minority with majority stake
    full_committee: HashSet<Validator>,      // 300 + 1 selected validator (301)
    epoch: u64,                              // Current epoch number
}

impl BFTCometRotation {
    /// Initialize rotation with minority and majority validator sets
    pub fn new(minority: Vec<Validator>, majority: Vec<Validator>, epoch: u64) -> Self {
        let minority_committee = VecDeque::from(minority);
        let majority_validators = majority.into_iter().collect::<HashSet<_>>();

        let mut full_committee = minority_committee.iter().cloned().collect::<HashSet<_>>();

        // Select 1 validator from majority outside minority for full committee
        if let Some(selected) = majority_validators.iter().find(|v| !full_committee.contains(v)) {
            full_committee.insert(selected.clone());
        }

        Self {
            minority_committee,
            majority_validators,
            full_committee,
            epoch,
        }
    }

    /// Rotate minority committee validators for next epoch
    pub fn rotate_minority(&mut self) {
        if let Some(rotated) = self.minority_committee.pop_front() {
            self.minority_committee.push_back(rotated);
        }
    }

    /// Update full committee based on rotated minority + new majority validator
    pub fn update_full_committee(&mut self) {
        self.full_committee = self.minority_committee.iter().cloned().collect();

        // Pick a majority validator not already in minority_committee
        if let Some(selected) = self.majority_validators.iter().find(|v| !self.full_committee.contains(v)) {
            self.full_committee.insert(selected.clone());
        }
    }

    /// Validate that all 301 validators submitted matching proof attestations
    pub fn validate_attestations(&self, attestations: &[ProofAttestation]) -> bool {
        if attestations.len() != FULL_COMMITTEE_SIZE {
            return false;
        }

        // Extract attestation hashes
        let attestation_hashes: HashSet<_> = attestations.iter().map(|a| &a.attestation_hash).collect();

        // All 301 must match the same attestation hash
        attestation_hashes.len() == 1
    }

    /// Simulate a full epoch step: rotate, update committee, and validate proofs
    pub fn epoch_step(&mut self, attestations: &[ProofAttestation]) -> bool {
        self.rotate_minority();
        self.update_full_committee();

        self.validate_attestations(attestations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_validator(id: &str) -> Validator {
        Validator {
            id: id.to_string(),
            last_active_epoch: 0,
            stake: 1000,
        }
    }

    #[test]
    fn test_rotation_and_validation() {
        let minority = (0..MINORITY_SIZE).map(|i| make_validator(&format!("minority{}", i))).collect();
        let majority = (MINORITY_SIZE..(MINORITY_SIZE + 10)).map(|i| make_validator(&format!("majority{}", i))).collect();

        let mut rotation = BFTCometRotation::new(minority, majority, 1);

        // Mock attestations all matching hash
        let attestation = ProofAttestation {
            vault_id: "vault1".to_string(),
            token: "dBTC".to_string(),
            size: 10,
            owner_hash: "ownerhash".to_string(),
            zk_root: "zkroot".to_string(),
            attestation_hash: "samehash".to_string(),
            timestamp: Utc::now().timestamp() as u64,
        };

        let attestations = vec![attestation.clone(); FULL_COMMITTEE_SIZE];

        assert!(rotation.validate_attestations(&attestations));

        // Run epoch step
        assert!(rotation.epoch_step(&attestations));
    }
}
