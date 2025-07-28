// Domex :: validator/validator_identity.rs
// Generates Poseidon-bound identity for validator using real-world anchors

use crate::utils::{poseidon_hash, read_gpu_fingerprint, read_cpu_id, get_uptime_hash, get_asn};
use crate::types::ValidatorIdentity;

/// Generates a unique, cryptographically verifiable identity hash for this validator node
pub fn build_validator_identity(pubkey: &str, stake_root: &str) -> ValidatorIdentity {
    // Collect physical node fingerprints
    let gpu_id = read_gpu_fingerprint();       // BIOS + PCI read
    let cpu_id = read_cpu_id();                // CPU + TPM if available
    let uptime_hash = get_uptime_hash();       // Monotonic uptime hash
    let asn = get_asn();                       // ASN via IP-based geo check

    // Concatenate identity fields into one input string
    let raw_input = format!("{}|{}|{}|{}|{}", pubkey, stake_root, gpu_id, cpu_id, asn);

    // Final poseidon hash identity
    let identity_hash = poseidon_hash(&raw_input);

    ValidatorIdentity {
        identity_hash,
        pubkey: pubkey.to_string(),
        stake_root: stake_root.to_string(),
        gpu_id,
        cpu_id,
        uptime_hash,
        asn,
    }
}
