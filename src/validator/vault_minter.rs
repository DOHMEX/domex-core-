// vault_minter.rs
// Domex vault minter for zk onboarding and withdrawal (validator-side logic)

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
        amount: u64,
    ) -> Result<(), VaultMintError> {
        if self.vaults.contains_key(&identity) {
            return Err(VaultMintError::DuplicateVault(identity));
        }
        if amount == 0 {
            return Err(VaultMintError::InvalidAmount);
        }
        self.vaults.insert(identity, amount);
        Ok(())
    }

    /// Burn (withdraw) tokens from a vault after ZK proof
    /// - Fails if vault does not exist or balance is insufficient
    pub fn burn_balance(
        &mut self,
        identity: &IdentityHash,
        amount: u64,
    ) -> Result<(), VaultMintError> {
        let balance = self.vaults.get_mut(identity).ok_or(VaultMintError::NotFound)?;
        if *balance < amount {
            return Err(VaultMintError::InsufficientBalance);
        }
        *balance -= amount;
        Ok(())
    }

    /// Credit (re-deposit) tokens to a vault — optional for future bridge inflow
    pub fn credit_balance(
        &mut self,
        identity: &IdentityHash,
        amount: u64,
    ) -> Result<(), VaultMintError> {
        let balance = self.vaults.entry(identity.clone()).or_insert(0);
        *balance += amount;
        Ok(())
    }

    /// Read vault balance (optional)
    pub fn get_balance(&self, identity: &IdentityHash) -> Option<u64> {
        self.vaults.get(identity).copied()
    }

    /// Check if vault has enough to fulfill a withdrawal
    pub fn has_sufficient_balance(&self, identity: &IdentityHash, amount: u64) -> bool {
        self.vaults.get(identity).map_or(false, |b| *b >= amount)
    }
}

/// Vault errors for minting and withdrawal
#[derive(Debug, Clone)]
pub enum VaultMintError {
    DuplicateVault(IdentityHash),
    InvalidAmount,
    NotFound,
    InsufficientBalance,
}
