// src/validator/validator_selection.rs

use std::collections::{HashMap, HashSet};
use crate::types::bftcomet::Validator;
use crate::common::poseidon_utils::poseidon_hash_u64;
use rand::{seq::IteratorRandom, thread_rng};

/// Configurable constants
pub const MINORITY_SIZE: usize = 300;
pub const ORIGIN_VALIDATOR_CAP: usize = 99_700;

/// Validator selection manager
pub struct ValidatorSelection {
    /// All validators indexed by ID
    pub all_validators: HashMap<String, Validator>,

    /// Set of minority validators (300 rotating judges)
    pub minority_committee: HashSet<String>,

    /// Set of origin submitters (e.g., up to 99,700 global validators)
    pub origin_validators: HashSet<String>,
}

impl ValidatorSelection {
    /// Initialize with full validator list
    pub fn new(all: Vec<Validator>) -> Self {
        let mut vs = ValidatorSelection {
            all_validators: HashMap::new(),
            minority_committee: HashSet::new(),
            origin_validators: HashSet::new(),
        };

        for v in all {
            vs.all_validators.insert(v.id.clone(), v);
        }

        // Select 300 minority validators based on stake and activity
        vs.minority_committee = vs.select_minority_committee();

        // All others are treated as origin submitters
        vs.origin_validators = vs
            .all_validators
            .keys()
            .filter(|id| !vs.minority_committee.contains(*id))
            .cloned()
            .collect();

        vs
    }

    /// Select top 300 validators by stake and recent activity
    fn select_minority_committee(&self) -> HashSet<String> {
        let mut ranked: Vec<&Validator> = self.all_validators.values().collect();
        ranked.sort_by(|a, b| {
            b.stake
                .cmp(&a.stake)
                .then_with(|| b.last_active_epoch.cmp(&a.last_active_epoch))
        });

        ranked
            .into_iter()
            .take(MINORITY_SIZE)
            .map(|v| v.id.clone())
            .collect()
    }

    /// Select one validator from origin pool using Poseidon(epoch) as seed
    pub fn select_origin_submitter(&self, epoch: u64) -> Option<String> {
        let seed = poseidon_hash_u64(epoch);
        let available: Vec<&String> = self.origin_validators.iter().collect();
        if available.is_empty() {
            None
        } else {
            let index = (seed % available.len() as u64) as usize;
            Some(available[index].clone())
        }
    }

    /// Rotate minority committee by removing lowest and inserting next best
    pub fn rotate_minority(&mut self) {
        if let Some(lowest) = self
            .minority_committee
            .iter()
            .min_by_key(|id| self.all_validators.get(*id).map(|v| v.stake).unwrap_or(0))
            .cloned()
        {
            self.minority_committee.remove(&lowest);

            let candidates: Vec<&Validator> = self
                .all_validators
                .values()
                .filter(|v| !self.minority_committee.contains(&v.id))
                .collect();

            if let Some(next_best) = candidates
                .into_iter()
                .max_by_key(|v| v.stake)
                .map(|v| v.id.clone())
            {
                self.minority_committee.insert(next_best);
            }

            // Recompute origin validators
            self.origin_validators = self
                .all_validators
                .keys()
                .filter(|id| !self.minority_committee.contains(*id))
                .cloned()
                .collect();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_validator(id: &str, stake: u64, epoch: u64) -> Validator {
        Validator {
            id: id.to_string(),
            stake,
            last_active_epoch: epoch,
        }
    }

    #[test]
    fn test_selection_logic() {
        let all_validators = (0..400)
            .map(|i| make_validator(&format!("v{}", i), 1000 - i as u64, 20))
            .collect::<Vec<_>>();

        let mut selection = ValidatorSelection::new(all_validators);
        assert_eq!(selection.minority_committee.len(), MINORITY_SIZE);
        assert_eq!(selection.origin_validators.len(), 100);

        if let Some(selected) = selection.select_origin_submitter(42) {
            assert!(selection.origin_validators.contains(&selected));
        }

        selection.rotate_minority();
        assert_eq!(selection.minority_committee.len(), MINORITY_SIZE);
    }
}
