// ======================================================
// proof_submitter.rs — Domex ZK Proof Submission Client
// ======================================================
//
// Handles automated delivery of onboarding proofs to Domex validators.
// Uses REST POST request to transmit proof + metadata payload.
//
// Requires: zk_config.rs, ZkOnboardingRequest structure

use crate::zk_client::zk_config::{
    VALIDATOR_PROOF_ENDPOINT,
    MAX_SUBMISSION_RETRIES,
    SUBMISSION_TIMEOUT_SECS,
};
use crate::types::zk_client::ZkOnboardingRequest;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use serde_json::json;
use std::time::Duration;

/// Represents the result of a proof submission attempt
#[derive(Debug)]
pub enum SubmissionResult {
    Success(String),   // Validator responded with OK or TxID
    Failure(String),   // Validator responded with error
    NetworkError(String),
}

/// Attempts to submit a ZK proof to the Domex validator endpoint
///
/// # Arguments
/// - `request`: Fully built onboarding request (proof + public inputs)
///
/// # Returns
/// - SubmissionResult enum with result or error message
pub fn submit_zk_proof(request: &ZkOnboardingRequest) -> SubmissionResult {
    let client = Client::builder()
        .timeout(Duration::from_secs(SUBMISSION_TIMEOUT_SECS))
        .build()
        .expect("Failed to build HTTP client");

    let payload = json!({
        "proof": base64::encode(&request.proof),
        "identity_hash": request.public_inputs.identity_hash.0.to_string(),
        "vault_id": request.public_inputs.vault_id,
        "zk_node_id": hex::encode(request.public_inputs.zk_node_id),
        "deposit_chain": request.public_inputs.deposit_chain,
        "deposit_tx_hash": request.public_inputs.deposit_tx_hash,
    });

    for attempt in 1..=MAX_SUBMISSION_RETRIES {
        let res = client
            .post(VALIDATOR_PROOF_ENDPOINT)
            .header(CONTENT_TYPE, "application/json")
            .json(&payload)
            .send();

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    match response.text() {
                        Ok(body) => return SubmissionResult::Success(body),
                        Err(_) => return SubmissionResult::Failure("Invalid response body".into()),
                    }
                } else {
                    let status = response.status().as_u16();
                    return SubmissionResult::Failure(format!("Validator rejected proof: HTTP {}", status));
                }
            }
            Err(e) => {
                if attempt == MAX_SUBMISSION_RETRIES {
                    return SubmissionResult::NetworkError(format!("Final attempt failed: {}", e));
                }
            }
        }
    }

    SubmissionResult::NetworkError("All submission attempts failed".into())
}
