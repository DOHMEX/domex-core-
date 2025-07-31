// Domex :: validator/withdraw_attestation.rs
// Signs Merkle withdrawal root and prepares validator attestation

use crate::types::{WithdrawAttestation, FinalizedWithdrawPackage};
use crate::utils::{poseidon_hash, sign_message};
use std::time::{SystemTime, UNIX_EPOCH};

/// Builds a validator attestation for a finalized withdrawal root.
/// This binds the validator to the withdrawal batch and vault changes.
///
/// This is shared with other validators or the external withdrawal pool.
pub fn build_withdraw_attestation(
    finalized_root: &str,
    vaults_touched: &[String],
    total_withdrawn: u64,
    validator_id: &str,
    validator_sk: &str,
) -> WithdrawAttestation {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time error")
        .as_secs();

    // Construct the withdrawal batch message to sign
    let message = format!(
        "{}|{}|{}|{}|{}",
        finalized_root,
        vaults_touched.join(","),
        total_withdrawn,
        validator_id,
        timestamp
    );

    let attestation_hash = poseidon_hash(&message);
    let signature = sign_message(validator_sk, &attestation_hash);

    WithdrawAttestation {
        withdraw_root: finalized_root.to_string(),
        validator_id: validator_id.to_string(),
        signature,
        vaults_touched: vaults_touched.to_vec(),
        total_withdrawn,
        attested_at: timestamp,
    }
}

/// Bundles the Merkle withdrawal root and attestation into a broadcastable package
pub fn package_finalized_withdrawal(
    root: &str,
    attestation: WithdrawAttestation,
) -> FinalizedWithdrawPackage {
    FinalizedWithdrawPackage {
        withdraw_root: root.to_string(),
        attestation,
    }
}
