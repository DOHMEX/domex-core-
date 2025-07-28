// src/types/bftcomet.rs

use serde::{Serialize, Deserialize};

/// Validator representation in the Domex global validator set
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Validator {
    /// Unique Poseidon identity or public key of the validator
    pub id: String,

    /// The last epoch in which the validator was active
    pub last_active_epoch: u64,

    /// Stake amount of the validator (used for ranking/selection)
    pub stake: u64,
}

/// Collection representing the rotating minority committee of validators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinorityCommittee(pub Vec<String>); // List of Validator IDs

/// Collection representing the majority validators eligible to join full committee
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajorityValidators(pub Vec<String>); // List of Validator IDs

/// Full committee consisting of the minority committee plus one selected majority validator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullCommittee(pub Vec<String>); // List of Validator IDs
