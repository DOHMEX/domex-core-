// src/validator/identity_registration.rs 
// Handles validator identity registration into the global system

use crate::utils::poseidon_hash; use crate::types::ValidatorIdentity; use std::collections::HashMap; use chrono::Utc;

// Simulated global registry (in-memory for now) static mut VALIDATOR_REGISTRY: Option<HashMap<String, ValidatorIdentity>> = None;

/// Registers a new validator with a Poseidon-based identity fingerprint pub fn register_validator(pubkey: &str, metadata: &str) -> ValidatorIdentity { let timestamp = Utc::now().timestamp() as u64; let identity_hash = poseidon_hash(&[pubkey, metadata, &timestamp.to_string()]);

let validator = ValidatorIdentity {
    pubkey: pubkey.to_string(),
    metadata: metadata.to_string(),
    joined_at: timestamp,
    identity_hash: identity_hash.clone(),
};

unsafe {
    VALIDATOR_REGISTRY
        .get_or_insert_with(HashMap::new)
        .insert(identity_hash.clone(), validator.clone());
}

validator

}

/// Retrieves a validator by its hashed identity pub fn get_validator_by_hash(hash: &str) -> Option<ValidatorIdentity> { unsafe { VALIDATOR_REGISTRY .as_ref() .and_then(|registry| registry.get(hash).cloned()) } }

