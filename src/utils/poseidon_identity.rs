// Domex :: utils/poseidon_identity.rs
// Generates a Poseidon-based identity hash from deterministic validator metadata

use crate::utils::poseidon_hash;
use blake2::Blake2s256;
use blake2::digest::{Update, VariableOutput};

/// Computes a Poseidon identity hash from normalized validator metadata
/// This hash is used as validator_id (e.g., Poseidon(pubkey || stake_root || gpu || ...))
pub fn generate_poseidon_identity(
    pubkey: &[u8],
    stake_root: &[u8],
    gpu_id: &[u8],
    cpu_id: &[u8],
    uptime_hash: &[u8],
    asn: &[u8],
) -> String {
    // Concatenate all binary inputs safely (fixed-length padding is optional)
    let mut hasher = Blake2s256::new();
    hasher.update(pubkey);
    hasher.update(stake_root);
    hasher.update(gpu_id);
    hasher.update(cpu_id);
    hasher.update(uptime_hash);
    hasher.update(asn);

    let intermediate = hasher.finalize_boxed(); // 32 bytes

    // Final Poseidon hash for identity
    poseidon_hash(&intermediate)
}
