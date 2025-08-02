// Domex :: validator/proof_attestation.rs
// Signs zk_root and prepares validator attestation for 301-node quorum using Poseidon-based hashing

use crate::types::{BatchAggregateResult, ProofAttestation};
use crate::utils::{poseidon_hash, poseidon_sign}; // <- Quantum-resistant signing
use std::time::{SystemTime, UNIX_EPOCH};

/// Builds a validator attestation over a verified zk_root batch.
/// The attestation binds validator identity to the zk proof result,
/// including proof metadata and vaults affected.
///
/// Sent to the 301-node validator quorum for attestation aggregation.
pub fn build_attestation(
    aggregated: &BatchAggregateResult,
    validator_id: &str,   // Poseidon(sk || zk_node_id)
    validator_sk: &[u8; 32], // Private key used in Poseidon-native signing
) -> ProofAttestation {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time is before UNIX epoch")
        .as_secs();

    // Construct deterministic message
    let message = format!(
        "{}|{}|{}|{}|{}|{}",
        aggregated.zk_root,
        aggregated.proof_count,
        aggregated.total_volume,
        aggregated.vaults_touched.join(","),
        validator_id,
        timestamp
    );

    // Hash the message using Poseidon
    let attestation_hash = poseidon_hash(&message);

    // Sign the hash using Poseidon-based scheme (quantum-resistant placeholder)
    let signature = poseidon_sign(validator_sk, &attestation_hash);

    ProofAttestation {
        zk_root: aggregated.zk_root.clone(),
        validator_id: validator_id.to_string(),
        signature,
        proof_count: aggregated.proof_count,
        total_volume: aggregated.total_volume,
        vaults_touched: aggregated.vaults_touched.clone(),
        attested_at: timestamp,
    }
}
