// ====================================================
// zk_config.rs — Domex ZK Client Constants & Defaults
// ====================================================
//
// Central configuration file for Domex ZK onboarding.
// Defines constants used across proof generation,
// submission, vault targeting, and chain integrations.

/// Default vault ID for onboarding (e.g., BTC/USDT board)
pub const DEFAULT_VAULT_ID: u64 = 777;

/// Default blockchain where deposit originated
pub const DEFAULT_CHAIN: &str = "BTC";

/// Domex global validator REST API endpoint for submitting ZK proofs
pub const VALIDATOR_PROOF_ENDPOINT: &str = "https://validators.domex.io/api/v1/submit-proof";

/// Fallback retry attempts for proof submission
pub const MAX_SUBMISSION_RETRIES: usize = 3;

/// Timeout (in seconds) for HTTP submission requests
pub const SUBMISSION_TIMEOUT_SECS: u64 = 10;

/// Whether to enable automatic delegation detection (if present in deposit address)
pub const ENABLE_DELEGATION_MODE: bool = true;

/// Whether to allow fallback to AutoVault if available
pub const ENABLE_AUTOVAULT_IF_AVAILABLE: bool = true;

/// Optional path to local cache file (for unclaimed deposits, logs, etc.)
pub const LOCAL_CACHE_PATH: &str = "./domex_cache.json";
