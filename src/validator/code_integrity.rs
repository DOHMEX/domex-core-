// src/validator/code_integrity.rs

use std::collections::HashMap;
use crate::types::validator::Validator;
use crate::common::poseidon_utils::poseidon_hash_bytes;

/// Tracks code hash of each validator
pub struct CodeIntegrityTracker {
    /// Mapping from validator ID to their known code hash
    pub validator_code_hashes: HashMap<String, String>,

    /// Slashed validators for modifying core logic
    pub slashed: Vec<String>,
}

impl CodeIntegrityTracker {
    /// Create new integrity tracker
    pub fn new() -> Self {
        Self {
            validator_code_hashes: HashMap::new(),
            slashed: vec![],
        }
    }

    /// Register a validator’s bytecode hash (initial submission)
    pub fn register_code_hash(&mut self, validator_id: &str, code_bytes: &[u8]) {
        let hash = poseidon_hash_bytes(code_bytes);
        self.validator_code_hashes.insert(validator_id.to_string(), hash);
    }

    /// Verify submitted bytecode is unchanged from previous registration
    pub fn verify_code_integrity(&mut self, validator_id: &str, new_code_bytes: &[u8]) -> bool {
        if let Some(stored_hash) = self.validator_code_hashes.get(validator_id) {
            let new_hash = poseidon_hash_bytes(new_code_bytes);
            if stored_hash != &new_hash {
                // Slashing condition
                self.slashed.push(validator_id.to_string());
                return false;
            }
            true
        } else {
            // First-time registration, auto-pass
            self.register_code_hash(validator_id, new_code_bytes);
            true
        }
    }

    /// Return slashed validators
    pub fn get_slashed(&self) -> &[String] {
        &self.slashed
    }
}
