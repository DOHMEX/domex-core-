// vault_minter.rs
// Domex vault minter for zk onboarding (validator-side logic)
// Mints vault to Poseidon-based identity hash after proof verification

use std::collections::HashMap;
use pasta_curves::Fp;
use crate::types::zk_client::ZkOnboardingPublicInputs;
use crate::types::poseidon_utils::IdentityHash;

/// In-memory vault registry (simulates Merkle + global state layer)
/// In production, this maps to Domex state root (via Verkle tree or validator backend)
pub struct VaultRegistry {
    // identity_hash → balance (e.g., onboarded token)
    pub vaults: HashMap<IdentityHash, u64>,
}

impl VaultRegistry {
    /// Create a new empty registry (per validator instance)
    pub fn new() -> Self {
        VaultRegistry {
            vaults: HashMap::new(),
        }
    }

    /// Mint a new vault to the given identity
    /// - Fails if vault already exists (to prevent double onboarding)
    pub fn mint_vault(
        &mut self,
        identity: IdentityHash,
        amount: u64, // assume token amount already known from deposit
    ) -> Result<(), VaultMintError> {
        if self.vaults.contains_key(&identity) {
            return Err(VaultMintError::DuplicateVault(identity));
        }
        self.vaults.insert(identity, amount);
        Ok(())
    }

    /// Read vault balance (optional)
    pub fn get_balance(&self, identity: &IdentityHash) -> Option<u64> {
        self.vaults.get(identity).copied()
    }
}

/// Vault minting errors
#[derive(Debug, Clone)]
pub enum VaultMintError {
    DuplicateVault(IdentityHash),
    InvalidAmount,
}
