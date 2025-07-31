// types/circuit_constants.rs
// Shared zk circuit-level constants and type definitions for Domex

use pasta_curves::Fp;

/// ========== Poseidon Hashing ==========

/// Number of inputs to Poseidon used for vault identity: Poseidon(sk, vault_id, zk_node_id)
pub const POSEIDON_IDENTITY_ARITY: usize = 3;

/// Number of inputs to Poseidon used for withdrawal proof binding: Poseidon(identity_hash, amount, nonce)
pub const POSEIDON_WITHDRAW_ARITY: usize = 3;

/// Domain-separated Poseidon labels for identity, withdrawal, and orders
pub mod poseidon_domains {
    pub const IDENTITY: &str = "domex.poseidon.identity";
    pub const WITHDRAW: &str = "domex.poseidon.withdraw";
    pub const ORDER: &str = "domex.poseidon.order";
}

/// Canonical output type for all Poseidon hashes in Domex
pub type VaultIdentityHash = Fp;

/// ========== zk Input Constraints ==========

/// zkNode ID length (bytes) used in Poseidon identity hash
pub const ZK_NODE_ID_BYTES: usize = 32;

/// Vault ID constraint — max number of bits (used in identity hash)
pub const MAX_VAULT_ID_BITS: usize = 64;

/// Secret key input — bit constraint for zk onboarding circuit
pub const SECRET_KEY_BITS: usize = 256;

/// ========== zk Proof Config ==========

/// Max zk proof byte size for onboarding (Groth16 or Plonky2)
pub const MAX_ZK_PROOF_BYTES: usize = 512;

/// Minimum valid deposit amount allowed for zk onboarding
pub const MIN_ONBOARD_AMOUNT: u64 = 10_000; // 10,000 sats or wei

/// Pasta curve modulus (decimal string), needed for field safety
pub const PALLAS_FIELD_MODULUS: &str =
    "28948022309329048855892746252171976963363056481941647379679742748393362948097";
