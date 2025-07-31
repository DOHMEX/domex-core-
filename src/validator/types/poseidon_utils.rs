// types/poseidon_utils.rs
// Shared constants and type aliases for Poseidon identity hashing in Domex

use pasta_curves::Fp;

/// Standardized alias for a Poseidon-derived identity or lock hash
pub type IdentityHash = Fp;

/// Poseidon input arity used in identity hash:
/// - onboarding: Poseidon(sk, vault_id, zk_node_id)
/// - withdrawal: Poseidon(sk, script_hash, withdraw_amount)
pub const POSEIDON_IDENTITY_ARITY: usize = 3;

/// Domain separation labels for Poseidon audit tracking and circuit consistency
pub mod poseidon_domains {
    /// For zk onboarding identity hash (vault binding)
    pub const IDENTITY: &str = "domex.poseidon.identity";

    /// For withdrawal-mode lock hash (script enforcement)
    pub const WITHDRAWAL: &str = "domex.poseidon.withdraw";

    /// For trading/order intents
    pub const ORDER: &str = "domex.poseidon.order";
}
