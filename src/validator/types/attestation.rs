// types/attestation.rs
// ======================
// Structs related to validator proof attestations

#[derive(Debug, Clone)]
pub struct NormalizedProof {
    pub vault_id: String,
    pub token: String,
    pub size: u64,
    pub owner_hash: String,
}

#[derive(Debug, Clone)]
pub struct ProofAttestation {
    pub vault_id: String,
    pub token: String,
    pub size: u64,
    pub owner_hash: String,
    pub zk_root: String,
    pub attestation_hash: String,
    pub timestamp: u64,
}
