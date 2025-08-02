// Domex :: validator/zk_proof_builder.rs
// Builds Phase 1 ZK-bound attestations for onboarding into vaults
// Used by validators to commit ZK-proof-backed onboarding state

use crate::types::NormalizedProof;
use crate::utils::poseidon_hash;
use chrono::Utc;

/// Represents a ZK proof attestation committed by validator nodes.
/// This attestation binds the onboarding proof to the vault, token, owner, and timestamp.
/// Validators use this as part of the Merkle root commitment step.
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

/// Builds a ZK-bound attestation from a normalized proof
/// 
/// This function performs the final commitment step by hashing:
/// - vault ID
/// - token type
/// - vault owner's identity hash
/// - zk_root (circuit commitment root)
/// - timestamp (UTC)
pub fn build_attestation(proof: &NormalizedProof, zk_root: &str) -> ProofAttestation {
    let timestamp = Utc::now().timestamp() as u64;

    let attestation_hash = poseidon_hash(&[
        &proof.vault_id,
        &proof.token,
        &proof.owner_hash,
        zk_root,
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
