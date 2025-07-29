// circuit_constants.rs
// Domex zk onboarding constants shared across client, circuit, and validator logic

use pasta_curves::Fp;

/// Poseidon input arity used for vault identity hash: Poseidon(sk, vault_id, zk_node_id)
pub const POSEIDON_IDENTITY_ARITY: usize = 3;

/// Bit length for secret key inputs in zk circuit (usually 256 bits for Pasta curve)
pub const SECRET_KEY_BITS: usize = 256;

/// Expected field modulus for the Pallas scalar field (Pasta curve)
pub const PALLAS_FIELD_MODULUS: &str =
    "28948022309329048855892746252171976963363056481941647379679742748393362948097";

/// Minimum onboardable token amount (safe default)
pub const MIN_ONBOARD_AMOUNT: u64 = 10_000; // e.g., sats, wei, etc.

/// Maximum vault ID size (bits) — used for zk constraint range checks
pub const MAX_VAULT_ID_BITS: usize = 64;

/// Validator-assigned zk_node_id byte length
pub const NODE_ID_BYTES: usize = 32;

/// Max zk proof byte size for onboarding (Groth16 default safety cap)
pub const MAX_ZK_PROOF_BYTES: usize = 512;

/// Poseidon personalization domains (for internal zk/circuit labels)
pub mod domains {
    pub const IDENTITY: &str = "domex.poseidon.identity";
    pub const WITHDRAW: &str = "domex.poseidon.withdraw";
    pub const ORDER: &str = "domex.poseidon.order";
}
