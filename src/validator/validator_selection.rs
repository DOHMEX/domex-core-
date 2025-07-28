// src/validator/validator_selection.rs

use std::collections::{HashSet, HashMap};
use crate::types::bftcomet::Validator;
use rand::{seq::IteratorRandom, thread_rng};

/// Configurable constants
pub const MINORITY_SIZE: usize = 300;

/// Validator selection manager
pub struct ValidatorSelection {
    /// All active validators indexed by id
    pub all_validators: HashMap<String, Validator>,

    /// Current minority committee (300 validators)
    pub minority_committee: HashSet<String>,

    /// Current majority validators (outside minority)
    pub majority_validators: HashSet<String>,
}

impl ValidatorSelection {
    /// Initialize with full validator set
    pub fn new(all_validators: Vec<Validator>) -> Self {
        let mut vs = ValidatorSelection {
            all_validators: HashMap::new(),
            minority_committee: HashSet::new(),
            majority_validators: HashSet::new(),
        };

        for v in all_validators {
            vs.all_validators.insert(v.id.clone(), v);
        }

        // Select minority committee at init
        vs.minority_committee = vs.select_minority_committee();

        // Define majority as those not in minority
        vs.majority_validators = vs
            .all_validators
            .keys()
            .filter(|id| !vs.minority_committee.contains(*id))
            .cloned()
            .collect();

        vs
    }

    /// Select the minority committee of 300 validators based on stake & activity
    fn select_minority_committee(&self) -> HashSet<String> {
        let mut rng = thread_rng();

        // Rank validators by stake (descending) and last_active_epoch (descending)
        let mut ranked: Vec<&Validator> = self.all_validators.values().collect();
        ranked.sort_by(|a, b| {
            b.stake
                .cmp(&a.stake)
                .then_with(|| b.last_active_epoch.cmp(&a.last_active_epoch))
        });

        // Select top MINORITY_SIZE validators deterministically
        ranked
            .iter()
            .take(MINORITY_SIZE)
            .map(|v| v.id.clone())
            .collect()
    }

    /// Select one validator from majority to join full committee
    pub fn select_majority_validator(&self) -> Option<String> {
        let mut rng = thread_rng();

        // Filter majority validators who are active (last_active_epoch recent)
        let active_majority: Vec<&String> = self
            .majority_validators
            .iter()
            .filter(|id| {
                if let Some(v) = self.all_validators.get(*id) {
                    v.last_active_epoch > 0 // e.g. active recently; customize as needed
                } else {
                    false
                }
            })
            .collect();

        active_majority.into_iter().choose(&mut rng).cloned()
    }

    /// Update minority committee on rotation (e.g. removing oldest, adding new)
    pub fn rotate_minority(&mut self) {
        // For simplicity, drop the lowest stake validator and add next highest not in minority
        if let Some(lowest) = self
            .minority_committee
            .iter()
            .min_by_key(|id| self.all_validators.get(*id).map(|v| v.stake).unwrap_or(0))
            .cloned()
        {
            self.minority_committee.remove(&lowest);

            // Find next highest stake validator outside minority
            let candidates: Vec<&Validator> = self
                .all_validators
                .values()
                .filter(|v| !self.minority_committee.contains(&v.id))
                .collect();

            if let Some(next_best) = candidates
                .iter()
                .max_by_key(|v| v.stake)
                .map(|v| v.id.clone())
            {
                self.minority_committee.insert(next_best);
            }

            // Update majority set accordingly
            self.majority_validators = self
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
    fn test_selection_and_rotation() {
        let mut all_validators = vec![];
        for i in 0..350 {
            all_validators.push(make_validator(&format!("v{}", i), 1000 - i as u64, 10));
        }

        let mut selection = ValidatorSelection::new(all_validators);

        // Initial minority committee size
        assert_eq!(selection.minority_committee.len(), MINORITY_SIZE);

        // Majority validator selected should be outside minority committee
        if let Some(selected) = selection.select_majority_validator() {
            assert!(!selection.minority_committee.contains(&selected));
        } else {
            panic!("No majority validator selected");
        }

        // Rotate minority committee and check size stays correct
        selection.rotate_minority();
        assert_eq!(selection.minority_committee.len(), MINORITY_SIZE);
    }
}
