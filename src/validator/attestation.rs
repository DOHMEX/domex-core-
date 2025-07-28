// Domex Validator Attestation Builder
// Generates attestation message after ZK proof verification

use crate::types::NormalizedProof;
use crate::utils::poseidon_hash;
use chrono::Utc;

/// Structure representing the final attestation data
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

/// Builds a ProofAttestation struct from a normalized, verified proof
pub fn build_attestation(proof: &NormalizedProof, zk_root: &str) -> ProofAttestation {
    let timestamp = Utc::now().timestamp() as u64;

    // Poseidon hash over all fields to create a unique attestation identifier
    let attestation_hash = poseidon_hash(&[
        &proof.vault_id,
        &proof.token,
        &proof.owner_hash,
        &zk_root.to_string(),
        &timestamp.to_string(),
    ]);

    ProofAttestation {
        vault_id: proof.vault_id.clone(),
        token: proof.token.clone(),
        size: proof.size,
        owner_hash: proof.owner_hash.clone(),
        zk_root: zk_root.to_string(),
        attestation_hash,
        timestamp,
    }
}
