// ==========================================================
// bftcomet_rotation.rs — Domex BFT-Comet Validator Rotation
// ==========================================================
//
// Handles validator committee rotation, attestation quorum checks,
// majority validator selection via Poseidon-hashed epoch,
// and supports scale-up to 99,700 validator submissions per epoch.
//

use std::collections::{HashSet, VecDeque};
use crate::validator::attestation::ProofAttestation;
use crate::common::poseidon_utils::poseidon_hash_u64;
use chrono::Utc;

/// Constants
pub const MINORITY_SIZE: usize = 300;
pub const FULL_COMMITTEE_SIZE: usize = 301;

/// Represents a validator in the rotation set
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Validator {
    pub id: String,              // Poseidon identity or pubkey
    pub last_active_epoch: u64, // Epoch participation tracking
    pub stake: u64,             // Used for ranking
}

/// Committee rotation and validation manager
pub struct BFTCometRotation {
    minority_committee: VecDeque<Validator>, // Fixed 300 judges
    majority_validators: HashSet<Validator>, // Global validator pool (up to 99,700)
    full_committee: HashSet<Validator>,      // 300 + 1 selected for attestation (301)
    epoch: u64,                              // Current epoch number
    epoch_start_ts: u64,                     // Timestamp in seconds
}

impl BFTCometRotation {
    /// Initialize new BFT-Comet rotation
    pub fn new(minority: Vec<Validator>, majority: Vec<Validator>, epoch: u64) -> Self {
        let minority_committee = VecDeque::from(minority);
        let majority_validators = majority.into_iter().collect::<HashSet<_>>();

        let mut full_committee = minority_committee.iter().cloned().collect::<HashSet<_>>();

        if let Some(selected) = Self::select_majority_validator(epoch, &majority_validators, &full_committee) {
            full_committee.insert(selected);
        }

        Self {
            minority_committee,
            majority_validators,
            full_committee,
            epoch,
            epoch_start_ts: Utc::now().timestamp() as u64,
        }
    }

    /// Deterministically selects a majority validator using Poseidon(epoch)
    fn select_majority_validator(
        epoch: u64,
        majority: &HashSet<Validator>,
        exclude: &HashSet<Validator>,
    ) -> Option<Validator> {
        let seed = poseidon_hash_u64(epoch);
        let available: Vec<_> = majority.iter().filter(|v| !exclude.contains(v)).cloned().collect();
        if available.is_empty() {
            None
        } else {
            let index = (seed % available.len() as u64) as usize;
            Some(available[index].clone())
        }
    }

    /// Rotate 1 validator out of the 300-judge committee
    pub fn rotate_minority(&mut self) {
        if let Some(rotated) = self.minority_committee.pop_front() {
            self.minority_committee.push_back(rotated);
        }
    }

    /// Rebuilds full committee after rotation
    pub fn update_full_committee(&mut self) {
        self.full_committee = self.minority_committee.iter().cloned().collect();

        if let Some(selected) = Self::select_majority_validator(self.epoch, &self.majority_validators, &self.full_committee) {
            self.full_committee.insert(selected);
        }
    }

    /// Returns the current full committee (301 validators)
    pub fn current_committee(&self) -> &HashSet<Validator> {
        &self.full_committee
    }

    /// Validates if all attestations match (301 required)
    pub fn validate_attestations(&self, attestations: &[ProofAttestation]) -> bool {
        if attestations.len() != FULL_COMMITTEE_SIZE {
            return false;
        }
        let hashes: HashSet<_> = attestations.iter().map(|a| &a.attestation_hash).collect();
        hashes.len() == 1
    }

    /// Detects validators who submitted wrong attestations (for slashing)
    pub fn detect_slashable_validators(&self, attestations: &[ProofAttestation]) -> Vec<Validator> {
        if attestations.len() != FULL_COMMITTEE_SIZE {
            return vec![];
        }

        let mut counts = std::collections::HashMap::new();
        for att in attestations {
            *counts.entry(&att.attestation_hash).or_insert(0) += 1;
        }

        let quorum_hash = counts.iter().max_by_key(|(_, count)| *count).map(|(h, _)| *h);
        if quorum_hash.is_none() {
            return vec![];
        }

        attestations.iter().enumerate()
            .filter_map(|(i, att)| {
                if &att.attestation_hash != quorum_hash.unwrap() {
                    self.full_committee.iter().nth(i).cloned()
                } else {
                    None
                }
            })
            .collect()
    }

    /// Step to the next epoch and revalidate
    pub fn epoch_step(&mut self, attestations: &[ProofAttestation]) -> bool {
        self.epoch += 1;
        self.epoch_start_ts = Utc::now().timestamp() as u64;
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
        let minority = (0..MINORITY_SIZE)
            .map(|i| make_validator(&format!("minority{}", i)))
            .collect();

        let majority = (MINORITY_SIZE..(MINORITY_SIZE + 99_400))
            .map(|i| make_validator(&format!("majority{}", i)))
            .collect();

        let mut rotation = BFTCometRotation::new(minority, majority, 1);

        let attestation = ProofAttestation {
            vault_id: "vault1".into(),
            token: "dBTC".into(),
            size: 100,
            owner_hash: "hash_owner".into(),
            zk_root: "zkroot".into(),
            attestation_hash: "onehash".into(),
            timestamp: Utc::now().timestamp() as u64,
        };

        let attestations = vec![attestation.clone(); FULL_COMMITTEE_SIZE];

        assert!(rotation.validate_attestations(&attestations));
        assert!(rotation.epoch_step(&attestations));
    }
    
