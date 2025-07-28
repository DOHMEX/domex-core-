// Domex :: validator/proof_attestation.rs
// Signs zk_root and prepares validator attestation for 301-node quorum

use crate::types::{BatchAggregateResult, ProofAttestation};
use crate::utils::{poseidon_hash, sign_message};
use std::time::{SystemTime, UNIX_EPOCH};

/// Builds a proof attestation that binds this validator to a specific zk_root,
/// along with vault metadata and validator identity.
///
/// This output is sent to the 300+1 validator quorum for consensus.
pub fn build_attestation(
    aggregated: &BatchAggregateResult,
    validator_id: &str,
    validator_sk: &str,
) -> ProofAttestation {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time before UNIX epoch")
        .as_secs();

    // Construct attestation content as a string to hash and sign
    let message = format!(
        "{}|{}|{}|{}|{}|{}",
        aggregated.zk_root,
        aggregated.proof_count,
        aggregated.total_volume,
        aggregated.vaults_touched.join(","),
        validator_id,
        timestamp
    );

    let attestation_hash = poseidon_hash(&message);
    let signature = sign_message(validator_sk, &attestation_hash);

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
