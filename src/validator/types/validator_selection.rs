// src/types/validator_selection.rs

use serde::{Serialize, Deserialize};
use crate::types::validator::Validator;

/// Represents a collection of Validators forming the minority committee
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinorityCommittee(pub Vec<Validator>);

/// Represents the set of Validators outside the minority committee (majority)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajorityValidators(pub Vec<Validator>);

/// Represents the full committee combining minority and one majority validator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullCommittee {
    /// Minority committee members
    pub minority: MinorityCommittee,
    /// Single majority validator added to minority committee
    pub majority_validator: Validator,
}
