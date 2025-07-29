// types/poseidon_utils.rs
// Shared constants and type aliases for Poseidon identity hashing in Domex

use pasta_curves::Fp;

/// Standardized alias for a Poseidon identity hash
pub type IdentityHash = Fp;

/// Standard Poseidon input count for Domex identity binding
pub const POSEIDON_IDENTITY_ARITY: usize = 3;

/// Domain separation prefixes for audit or circuit-level labels (optional)
pub mod poseidon_domains {
    pub const IDENTITY: &str = "domex.poseidon.identity";
    pub const WITHDRAWAL: &str = "domex.poseidon.withdraw";
    pub const ORDER: &str = "domex.poseidon.order";
}
