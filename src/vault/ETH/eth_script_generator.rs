// Domex :: eth_script_generator.rs
// Generates Poseidon-locked ETH vault script for binding identity to ETH transfers

use crate::poseidon_utils::poseidon_hash2;
use ethers::types::Address;

/// Represents a Domex ETH vault lock script
#[derive(Debug, Clone)]
pub struct EthVaultScript {
    pub owner_identity_hash: [u8; 32], // Poseidon(private_key || vault_id)
    pub eth_address: Address,         // ETH address bound to this vault
}

impl EthVaultScript {
    /// Generate a new ETH vault script based on the owner's identity and address
    pub fn new(private_key_bytes: &[u8; 32], vault_id: &str, eth_address: Address) -> Self {
        let vault_id_bytes = vault_id.as_bytes();
        let mut combined = [0u8; 64];
        combined[..32].copy_from_slice(private_key_bytes);
        combined[32..][..vault_id_bytes.len()].copy_from_slice(vault_id_bytes);

        let owner_identity_hash = poseidon_hash2(&combined[..32], &combined[32..]);

        Self {
            owner_identity_hash,
            eth_address,
        }
    }

    /// Serialize script logic (mocked here for audit logs — Domex uses proof not deployment)
    pub fn to_script_string(&self) -> String {
        format!(
            "ETHVaultScript {{ identity_hash: 0x{}, eth_address: {} }}",
            hex::encode(self.owner_identity_hash),
            self.eth_address
        )
    }
}
