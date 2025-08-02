// Domex :: eth_address_utils.rs
// Poseidon-based identity and vault script utils for ETH vault compatibility

use ark_bn254::Fr;
use serde::{Deserialize, Serialize};
use crate::poseidon_utils::{poseidon_hash2, poseidon_hash3};

/// Represents a Domex ETH vault identity binding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthVaultIdentity {
    pub vault_id: String,         // Unique vault (e.g., "vault-eth-01")
    pub zk_node_id: String,       // Node ID handling the vault
    pub public_key: [u8; 32],     // Prover's ETH-compatible key (compressed)
}

impl EthVaultIdentity {
    /// Computes identity hash: Poseidon(public_key || vault_id)
    pub fn identity_hash(&self) -> [u8; 32] {
        poseidon_hash2(&self.public_key, &self.vault_id.as_bytes())
    }

    /// Computes vault binding hash: Poseidon(identity_hash || zk_node_id)
    pub fn bound_identity(&self) -> [u8; 32] {
        let id_hash = self.identity_hash();
        poseidon_hash2(&id_hash, &self.zk_node_id.as_bytes())
    }
}

/// Computes ETH vault script hash (e.g., non-custodial logic code as bytecode)
pub fn compute_eth_vault_script_hash(script_bytes: &[u8]) -> [u8; 32] {
    poseidon_hash3(script_bytes, b"ETH", b"vault")
}

/// Builds a vault pool hash used in Merkle leaf tracking
pub fn compute_eth_pool_hash(
    identity_hash: [u8; 32],
    script_hash: [u8; 32],
    balance_wei: u128,
) -> [u8; 32] {
    // Convert balance to byte array
    let balance_bytes = balance_wei.to_be_bytes();
    poseidon_hash3(&identity_hash, &script_hash, &balance_bytes)
}
