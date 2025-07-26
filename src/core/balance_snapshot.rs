// ==================================================
// balance_snapshot.rs — Merkle-Ready Delta Extractor
// ==================================================

use crate::types::{BalanceChange, MerkleDelta};

/// Converts a list of balance changes into a Merkle-compatible delta
///
/// This is called *after* a successful trade and is passed to the ZK proof generator.
///
/// Example input:
/// - Alice: -0.5 dBTC
/// - Bob: +0.5 dBTC
///
/// Output:
/// - MerkleDelta { identity, token, before, after }
pub fn generate_balance_delta(changes: &[BalanceChange]) -> Vec<MerkleDelta> {
    changes
        .iter()
        .map(|c| {
            // Snapshot structure depends on external state; assume previous balances are known upstream
            // This example assumes the balance delta already happened in vault state
            let before = 0; // Placeholder — real implementation would query snapshot history
            let after = before + c.delta;

            MerkleDelta {
                identity: c.identity.clone(),
                token: c.token.clone(),
                before,
                after,
            }
        })
        .collect()
}
