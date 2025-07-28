// =================================================================
// zk/proof_dispatch.rs : ZK Proof Dispatch Trigger (Raft-aware)
// ==================================================================

use crate::zk::proof_generator::generate_proof;
use crate::types::zk::ZkProofInput;
use crate::infra::raft_context::is_raft_leader;

/// Attempts to dispatch a ZK proof if this node is the Raft leader.
/// Other nodes do nothing.
///
/// # Arguments
/// * `input` - Prepared ZK proof input generated after a trade commit.
pub fn dispatch_if_leader(input: ZkProofInput) {
    if is_raft_leader() {
        println!("[ZK Dispatch] Raft leader — generating proof...");
        match generate_proof(input.clone()) {
            Ok(proof_hash) => {
                println!("[ZK Dispatch] Proof generated: {}", proof_hash);
                // TODO: Send proof to validator layer or enqueue to L1
            }
            Err(e) => {
                eprintln!("[ZK Dispatch] Proof generation failed: {}", e);
            }
        }
    } else {
        println!("[ZK Dispatch] Not leader — skipping proof dispatch.");
        // Optionally store input locally for fallback
        // crate::zk::proof_cache::store(input);
    }
}
