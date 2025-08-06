// ==========================================================
// zk_constants.rs — ZK Circuit Constants for Domex
// ==========================================================
//
// Defines global zero-knowledge proof parameters used across:
// - Vault onboarding
// - Trade matching proof
// - Withdrawals and Merkle root transitions
// - Poseidon input/output padding for uniformity
//

/// Maximum number of trades per batch in ZK circuit
pub const MAX_TRADES_PER_BATCH: usize = 10_000;

/// Max vaults supported per proof aggregation window
pub const MAX_VAULTS_PER_PROOF: usize = 64;

/// Number of Poseidon inputs for standard hashing
pub const POSEIDON_INPUT_WIDTH: usize = 3;

/// Merkle tree height used for vault state roots
pub const VAULT_MERKLE_TREE_HEIGHT: usize = 32;

/// Merkle tree height for fuel burn registry
pub const FUEL_MERKLE_TREE_HEIGHT: usize = 16;

/// Max number of validator attestations per block
pub const MAX_VALIDATOR_ATTESTATIONS: usize = 301;

/// Minimum allowed proof size (compressed)
pub const MIN_PROOF_BYTES: usize = 512;

/// Maximum allowed proof size (compressed)
pub const MAX_PROOF_BYTES: usize = 2048;

/// Minimum acceptable ZK proof verification time (ms)
pub const MIN_GPU_VERIFICATION_MS: u64 = 1;

/// Maximum allowed ZK proof verification time (ms)
pub const MAX_GPU_VERIFICATION_MS: u64 = 100;

/// Number of decimals for normalized trade price/amount
pub const ZK_DECIMAL_PRECISION: u64 = 1_000_000;

/// ZK onboarding field width (e.g., 2 inputs: asset, identity)
pub const ZK_ONBOARDING_INPUT_WIDTH: usize = 2;

/// ZK withdrawal field width (e.g., 3 inputs: UTXO, owner, sig)
pub const ZK_WITHDRAWAL_INPUT_WIDTH: usize = 3;
