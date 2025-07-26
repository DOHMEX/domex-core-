// =======================================================
// vault_registry.rs — Domex Vault Metadata + Activation
// =======================================================

use std::collections::HashMap;

/// Unique identifier for a trading pair vault (e.g. "BTC/USDT")
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct VaultPair(pub String);

/// Vault status for trading lifecycle management
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VaultStatus {
    Active,
    Paused,
    Deprecated,
}

/// Per-vault trading rules and constraints
#[derive(Clone, Debug)]
pub struct VaultMetadata {
    pub tick_size: u64,         // Minimum price increment (e.g. 100 = $1.00)
    pub lot_size: u64,          // Minimum order size (e.g. 10_000 = 0.01 BTC)
    pub max_delta_bps: u64,     // Max allowed deviation in BPS (200 = 2%)
    pub base_token: String,     // e.g. "BTC"
    pub quote_token: String,    // e.g. "USDT"
    pub liquidity_price: u64,   // Global VWAP or oracle anchor
    pub status: VaultStatus,    // Active, Paused, Deprecated
}

/// Holds vault-level configs + user activations
#[derive(Default)]
pub struct VaultRegistry {
    pub metadata_map: HashMap<VaultPair, VaultMetadata>,
    pub activation_map: HashMap<(VaultPair, String), bool>, // (vault_id, user_hash) → active
}

impl VaultRegistry {
    /// Create a fresh, empty registry
    pub fn new() -> Self {
        VaultRegistry {
            metadata_map: HashMap::new(),
            activation_map: HashMap::new(),
        }
    }

    /// Register a new vault with its parameters
    pub fn register_vault(&mut self, pair: VaultPair, metadata: VaultMetadata) {
        self.metadata_map.insert(pair, metadata);
    }

    /// Activate a vault for a user after ZK onboarding proof
    pub fn activate_vault(&mut self, pair: &VaultPair, identity_hash: &str) {
        self.activation_map
            .insert((pair.clone(), identity_hash.to_string()), true);
    }

    /// Check if a vault is active for a given identity
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

    /// Update the global liquidity anchor (VWAP/oracle)
    pub fn update_liquidity_price(&mut self, pair: &VaultPair, new_price: u64) {
        if let Some(metadata) = self.metadata_map.get_mut(pair) {
            metadata.liquidity_price = new_price;
        }
    }

    /// Fetch the latest liquidity anchor for trading checks
    pub fn get_liquidity_price(&self, pair: &VaultPair) -> Option<u64> {
        self.metadata_map.get(pair).map(|meta| meta.liquidity_price)
    }
}
