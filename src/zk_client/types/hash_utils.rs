// types/hash_utils.rs
// Shared field-safe types and constants for Domex Poseidon and zk hash logic

use pasta_curves::Fp;

/// Field wrapper type alias (optional, for clarity in interfaces)
pub type FieldElement = Fp;

/// Length of Poseidon hash input array used for identity
pub const IDENTITY_HASH_INPUT_LEN: usize = 3;

/// Poseidon domain separation tags (optional, for future flexibility)
pub mod domain_tags {
    pub const IDENTITY_HASH: &str = "domex.identity";
    pub const VAULT_HASH: &str = "domex.vault";
    pub const WITHDRAWAL_HASH: &str = "domex.exit";
}
