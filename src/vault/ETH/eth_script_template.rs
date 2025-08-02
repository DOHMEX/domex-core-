// Domex :: eth_script_template.rs
// ETH vault script templates — used to generate Poseidon-based ETH vault bindings

use ethers::types::{Address, U256};
use crate::poseidon_utils::poseidon_hash2;

/// Represents a Domex ETH vault script template
/// This is *not* a smart contract — it's a hash-bound logic template
#[derive(Debug, Clone)]
pub struct EthVaultScript {
    pub vault_id: String,
    pub eth_address: Address,
    pub poseidon_hash: [u8; 32], // Poseidon(vault_id || eth_address)
}

impl EthVaultScript {
    /// Create a new ETH vault script template
    pub fn new(vault_id: impl Into<String>, eth_address: Address) -> Self {
        let vault_str = vault_id.into();
        let hash = poseidon_hash2(vault_str.as_bytes(), eth_address.as_bytes());

        Self {
            vault_id: vault_str,
            eth_address,
            poseidon_hash: hash,
        }
    }

    /// Returns a serialized script hash for off-chain indexing or Merkle inclusion
    pub fn to_hash(&self) -> [u8; 32] {
        self.poseidon_hash
    }

    /// Render human-readable summary (used for diagnostics or audit)
    pub fn render_summary(&self) -> String {
        format!(
            "ETH Vault Script:\n- Vault ID: {}\n- ETH Address: {:?}\n- Script Hash (Poseidon): 0x{}",
            self.vault_id,
            self.eth_address,
            hex::encode(self.poseidon_hash)
        )
    }
}
