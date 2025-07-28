// Domex :: validator/attestation_checker.rs
// Validates received attestation from any of the 301 quorum validators

use crate::types::ProofAttestation;
use crate::utils::{poseidon_hash, verify_signature};

/// Verifies that a received ProofAttestation is cryptographically valid:
/// - Signature matches the validator's identity
/// - zk_root is properly formatted
/// - Vaults list is non-empty
/// - Attestation timestamp is within a sane range
pub fn validate_attestation(attestation: &ProofAttestation, validator_pubkey: &str) -> bool {
    // Reconstruct the attested message hash
    let message = format!(
        "{}|{}|{}|{}|{}|{}",
        attestation.zk_root,
        attestation.proof_count,
        attestation.total_volume,
        attestation.vaults_touched.join(","),
        attestation.validator_id,
        attestation.attested_at
    );

    let message_hash = poseidon_hash(&message);

    // Ensure signature matches the validator’s public key
    let is_valid_signature = verify_signature(validator_pubkey, &message_hash, &attestation.signature);

    // Sanity check: zk_root must not be empty and vaults must exist
    let zk_root_ok = !attestation.zk_root.is_empty();
    let vaults_ok = !attestation.vaults_touched.is_empty();

    // Optional: timestamp sanity check (must be within ±12h of current time)
    let now = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(t) => t.as_secs(),
        Err(_) => return false,
    };
    let time_delta = now.abs_diff(attestation.attested_at);
    let timestamp_ok = time_delta <= 43200; // 12 hours

    is_valid_signature && zk_root_ok && vaults_ok && timestamp_ok
}
