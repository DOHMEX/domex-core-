// Domex :: types/validator_identity.rs
// Struct defining a validator's cryptographic identity used in zk attestation

use serde::{Deserialize, Serialize};

/// A unique cryptographic identity for each validator, bound to real-world hardware and network metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorIdentity {
    pub identity_hash: String,   // Poseidon(pubkey + stake_root + GPU + CPU + ASN)
    pub pubkey: String,          // Validator’s public staking key
    pub stake_root: String,      // Commitment to validator’s locked stake
    pub gpu_id: String,          // GPU fingerprint (BIOS, PCI)
    pub cpu_id: String,          // CPU/TPM identifier
    pub uptime_hash: String,     // Monotonic uptime snapshot hash
    pub asn: String,             // Autonomous System Number (network origin)
}
