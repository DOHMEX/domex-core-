// Domex :: validator/zk_challenge_seed.rs
// Generates unique Poseidon-based seed for each epoch to prevent ZK replay attacks

use crate::utils::poseidon_hash;

/// Generates a unique benchmark challenge seed for the current epoch.
/// This seed is passed into all validator ZK circuits to prevent replay or reuse.
///
/// # Parameters
/// - `epoch_number`: current epoch index (u64)
///
/// # Returns
/// - `String`: Poseidon hash of the seed input, used as challenge
pub fn generate_epoch_seed(epoch_number: u64) -> String {
    let seed_input = format!("domex_epoch_{}", epoch_number);
    poseidon_hash(&seed_input)
}
