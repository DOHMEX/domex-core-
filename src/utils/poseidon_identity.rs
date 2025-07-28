// Domex :: utils/poseidon_identity.rs
// Generates a Poseidon-based identity hash for validators and vaults

use crate::utils::poseidon_hash;

/// Constructs a unique validator identity hash from individual identity components.
/// Inputs must be deterministic and collision-resistant.
pub fn generate_poseidon_identity(
    pubkey: &str,
    stake_root: &str,
    gpu_id: &str,
    cpu_id: &str,
    uptime_hash: &str,
    asn: &str,
) -> String {
    // Normalize all fields into a single delimited string
    let input = format!(
        "{}|{}|{}|{}|{}|{}",
        pubkey, stake_root, gpu_id, cpu_id, uptime_hash, asn
    );

    // Poseidon hash the concatenated input
    poseidon_hash(&input)
}
