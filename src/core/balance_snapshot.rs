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
        .map(|change| {
            let identity = change.identity.clone(); //  Poseidon identity hash
            let token = change.token.clone();       //  Token symbol (e.g., dBTC)
            let delta_value = change.delta;         //  Change in token balance

            // In production, these would be fetched from historical vault state snapshots.
            // For now, we simulate with a zero-baseline and delta math.
            let before_balance: i64 = 0; // Placeholder — replace with snapshot lookup
            let after_balance = before_balance + delta_value;

            MerkleDelta {
                identity,
                token,
                before: before_balance,
                after: after_balance,
            }
        })
        .collect()
}
