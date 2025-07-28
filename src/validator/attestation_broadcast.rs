// Domex: Attestation Broadcast with ZK Verification (Plonky2) // This module sends finalized proof attestations to the validator quorum without gossip, // relying on Plonky2 recursive proof verification instead of signature-based systems.

use crate::types::zk_attestation::ZkAttestationPackage; use crate::utils::plonky2_verifier::verify_recursive_proof; use crate::validator::committee::get_current_quorum; use crate::validator::quorum_commit::QuorumCommitState; use crate::validator::zk_state::store_attestation_state; use std::collections::HashMap; use std::net::SocketAddr; use std::time::Duration; use tokio::net::TcpStream; use tokio::io::AsyncWriteExt; use tokio::time::timeout;

/// Timeout for validator response (in milliseconds) const VALIDATOR_TIMEOUT_MS: u64 = 150;

/// Sends a verified attestation to all quorum peers and waits for confirmation pub async fn broadcast_attestation_plonky2( package: ZkAttestationPackage, commit_state: &mut QuorumCommitState, ) -> Result<(), String> { // Verify the proof with Plonky2 before broadcasting let is_valid = verify_recursive_proof(&package.zk_proof, &package.attestation_hash); if !is_valid { return Err("ZK proof failed Plonky2 verification".to_string()); }

// Get current validator quorum
let quorum_peers = get_current_quorum();
let mut responses: HashMap<SocketAddr, bool> = HashMap::new();

for peer in quorum_peers.iter() {
    let peer_clone = peer.clone();
    let data = bincode::serialize(&package).map_err(|_| "Failed to serialize attestation")?;

    match timeout(Duration::from_millis(VALIDATOR_TIMEOUT_MS), TcpStream::connect(peer_clone)).await {
        Ok(Ok(mut stream)) => {
            if let Err(_) = stream.write_all(&data).await {
                responses.insert(peer_clone, false);
            } else {
                responses.insert(peer_clone, true);
            }
        }
        _ => {
            responses.insert(peer_clone, false);
        }
    }
}

let success_count = responses.values().filter(|&&ok| ok).count();
if success_count >= commit_state.required_quorum() {
    store_attestation_state(&package);
    Ok(())
} else {
    Err("Not enough validator confirmations".to_string())
}

}

