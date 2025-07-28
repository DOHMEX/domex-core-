// src/validator/types/identity_registration.rs
// Type definition for validator identity registration and lookup

#[derive(Debug, Clone)]
pub struct ValidatorIdentity {
    pub pubkey: String,
    pub metadata: String,
    pub timestamp: u64,
    pub identity_hash: String,
}
