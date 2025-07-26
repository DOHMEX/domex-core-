use serde::{Serialize, Deserialize};

/// Represents a unique vault trading pair like "BTC/USDT"
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct VaultPair(pub String);

/// Defines trading rules for a specific vault
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VaultMetadata {
    pub tick_size: u64,         // Minimum price step (e.g., 100 = $1.00)
    pub max_delta_bps: u64,     // Max price delta in BPS (e.g., 200 = 2%)
    pub base_token: String,     // e.g., "BTC"
    pub quote_token: String,    // e.g., "USDT"
}
