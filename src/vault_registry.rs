// =======================================================
// vault_registry.rs — Domex Vault Metadata + Activation
// =======================================================

use std::collections::HashMap;

/// Unique identifier for a trading pair vault (e.g. "BTC/USDT")
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct VaultPair(pub String);

/// Per-vault trading rules and constraints
#[derive(Clone, Debug)]
pub struct VaultMetadata {
    pub tick_size: u64,         // Minimum price movement (e.g. 100 = $1.00)
    pub max_delta_bps: u64,     // Max delta in basis points (e.g. 200 = 2%)
    pub base_token: String,
    pub quote_token: String,
}

/// Vault activation status — tracks which Poseidon identities are allowed to trade
#[derive(Default)]
pub struct VaultRegistry {
    pub metadata_map: HashMap<VaultPair, VaultMetadata>,
    pub activation_map: HashMap<(VaultPair, String), bool>, // (vault, identity_hash) → active?
}

impl VaultRegistry {
    /// Register a new vault and its parameters
    pub fn register_vault(&mut self, pair: VaultPair, metadata: VaultMetadata) {
        self.metadata_map.insert(pair, metadata);
    }

    /// Activate a vault for a given user (after ZK onboarding proof)
    pub fn activate_vault(&mut self, pair: &VaultPair, identity_hash: &str) {
        self.activation_map
            .insert((pair.clone(), identity_hash.to_string()), true);
    }

    /// Check if a vault is active for a user
    pub fn is_vault_active(&self, pair: &VaultPair, identity_hash: &str) -> bool {
        self.activation_map
            .get(&(pair.clone(), identity_hash.to_string()))
            .copied()
            .unwrap_or(false)
    }

    /// Retrieve vault trading rules
    pub fn get_metadata(&self, pair: &VaultPair) -> Option<&VaultMetadata> {
        self.metadata_map.get(pair)
    }
}
