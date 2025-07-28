// Domex :: types/attestation_checker.rs
// Optional support types for attestation validation (if needed)

use serde::{Deserialize, Serialize};

/// Result of a validator attestation check
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AttestationStatus {
    Valid,
    InvalidSignature,
    EmptyZkRoot,
    EmptyVaultList,
    TimestampOutOfRange,
}
