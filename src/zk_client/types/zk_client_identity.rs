// types/zk_client.rs
// Shared ZK client types for Domex onboarding

use pasta_curves::Fp;
use serde::{Deserialize, Serialize};

/// Public inputs to the Domex zk onboarding circuit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkOnboardingPublicInputs {
    pub identity_hash: Fp,
    pub vault_id: u64,
    pub zk_node_id: [u8; 32],
    pub pk_x: Fp,     // X coordinate of the public key
    pub pk_y: Fp,     // Y coordinate of the public key
    pub deposit_chain: String,       // e.g., "BTC", "ETH"
    pub deposit_tx_hash: String,     // transaction hash of native deposit
}

/// Raw private input (used internally, not exposed)
#[derive(Debug, Clone)]
pub struct ZkPrivateInput {
    pub sk_bytes: [u8; 32],         // secret key
}

/// Wrapper to send full zk onboarding request to Domex validators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkOnboardingRequest {
    pub proof: Vec<u8>,                     // zk-SNARK proof bytes
    pub public_inputs: ZkOnboardingPublicInputs,
}
