// types/circuit_interface.rs
// Shared types for circuit <-> zk-client interface in Domex onboarding

use pasta_curves::Fp;

/// Flattened, verified zk inputs passed into the Domex ZK circuit
#[derive(Debug, Clone)]
pub struct CircuitInputs {
    pub sk: Fp,
    pub vault_id: u64,
    pub zk_node_id: [u8; 32],
    pub pk_x: Fp,
    pub pk_y: Fp,
    pub identity_hash: Fp,
    pub deposit_chain: String,
    pub deposit_tx_hash: String,
}

/// Output from circuit execution — wraps Groth16/Plonk zk proof
#[derive(Debug, Clone)]
pub struct CircuitProof {
    pub proof_bytes: Vec<u8>,      // SNARK proof bytes
}
