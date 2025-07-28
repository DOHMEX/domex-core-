// Domex Phase 1 - Validator Proof Intake Handler
// Receives and parses ZK proof submissions from local nodes

use crate::types::{IncomingProof, NormalizedProof};
use crate::utils::{is_valid_vault, is_valid_poseidon_hash};

/// Main entry for handling an incoming ZK proof request from a local node
pub fn handle_incoming_proof(proof: IncomingProof) -> Result<NormalizedProof, String> {
    // Basic validation
    if !is_valid_vault(&proof.vault_id) {
        return Err("Invalid vault ID".to_string());
    }

    if !is_valid_poseidon_hash(&proof.owner_hash) {
        return Err("Malformed Poseidon identity hash".to_string());
    }

    if proof.token.is_empty() || proof.size == 0 {
        return Err("Token or size missing".to_string());
    }

    // Normalize and return structured proof for verification
    Ok(NormalizedProof {
        vault_id: proof.vault_id,
        token: proof.token,
        size: proof.size,
        zk_payload: proof.zk_payload,
        owner_hash: proof.owner_hash,
        timestamp: proof.timestamp,
    })
}
