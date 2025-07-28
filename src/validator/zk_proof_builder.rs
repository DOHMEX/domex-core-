// Domex Validator ZK Proof Builder (Phase 1)
// src/validator/zk_proof_builder.rs

use crate::types::NormalizedProof;
use crate::utils::poseidon_hash;
use chrono::Utc;

/// Final proof attestation struct used in Phase 1
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

/// Builds a ZK-bound attestation from a verified proof
pub fn build_attestation(proof: &NormalizedProof, zk_root: &str) -> ProofAttestation {
    let timestamp = Utc::now().timestamp() as u64;

    // Build poseidon hash commitment
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
