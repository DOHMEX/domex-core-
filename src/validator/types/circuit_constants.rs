// types/circuit_constants.rs
// Shared type-level access to zk circuit constants for Domex

use pasta_curves::Fp;

/// Poseidon hash arity for vault identity
pub const POSEIDON_IDENTITY_ARITY: usize = 3;

/// zkNode ID length (used in identity hash)
pub const ZK_NODE_ID_BYTES: usize = 32;

/// Vault ID constraint — max bits
pub const MAX_VAULT_ID_BITS: usize = 64;

/// Secret key constraint — max bits
pub const SECRET_KEY_BITS: usize = 256;

/// zk-proof max size for onboarding
pub const MAX_ZK_PROOF_BYTES: usize = 512;

/// Minimum valid deposit amount for onboarding (e.g., in sats or wei)
pub const MIN_ONBOARD_AMOUNT: u64 = 10_000;

/// Pallas field modulus (decimal string)
pub const PALLAS_FIELD_MODULUS: &str =
    "28948022309329048855892746252171976963363056481941647379679742748393362948097";

/// Personalization domains for Poseidon zk usage
pub mod poseidon_domains {
    pub const IDENTITY: &str = "domex.poseidon.identity";
    pub const WITHDRAW: &str = "domex.poseidon.withdraw";
    pub const ORDER: &str = "domex.poseidon.order";
}

/// Type alias for zk-based vault identity (Poseidon output)
pub type VaultIdentityHash = Fp;
