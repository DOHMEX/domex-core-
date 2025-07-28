// ============================================================
// zk/proof_dispatch.rs : Domex ZK Proof Trigger (Post-Raft)
// ============================================================

use crate::zk::proof_generator::generate_and_submit_proof;
use crate::zk::proof_cache::store_local_proof;
use crate::infra::raft_context::is_local_leader;
use crate::types::TradeResult;

/// Dispatches ZK proof after a trade is committed via Raft.
/// Raft leader: generates and submits proof
/// Follower: caches proof locally in case of failover
pub fn dispatch_zk_proof(trade: TradeResult) {
    if is_local_leader() {
        println!("[ZKP] I am Raft leader — generating proof for vault {}", trade.vault_id);
        if let Err(e) = generate_and_submit_proof(&trade) {
            eprintln!("[ZKP] Error submitting proof: {}", e);
        }
    } else {
        println!("[ZKP] Not Raft leader — caching backup proof for vault {}", trade.vault_id);
        if let Err(e) = store_local_proof(&trade) {
            eprintln!("[ZKP] Error caching proof: {}", e);
        }
    }
}
