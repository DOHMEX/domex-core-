// ======================================================
// zk_watcher.rs — Domex ZK Deposit Watcher (Client-Side)
// ======================================================
//
// Scans native chain for deposits, maps them to ZK identities,
// and stores them for future vault onboarding or AutoVault claim.
// Fully compatible with Plonky2 + Poseidon identity flow.

use crate::zk_client::cold_deposit::ColdDeposit;
use crate::types::common::{Token, ZkIdentity, DepositAddress};
use std::collections::HashMap;

/// Simulated deposit log from a native chain watcher
pub struct DetectedDeposit {
    pub token: Token,
    pub amount: f64,
    pub tx_hash: String,
    pub to_address: DepositAddress,
    pub timestamp: Option<u64>,
}

/// Cold deposit pool (used before proof is submitted or AutoVault triggers)
#[derive(Default)]
pub struct ColdDepositPool {
    /// Maps deposit address to deposit record
    pub deposits: HashMap<DepositAddress, ColdDeposit>,
}

impl ColdDepositPool {
    /// Registers a newly detected deposit if it's not already in the pool
    pub fn register_deposit(
        &mut self,
        detected: DetectedDeposit,
        expected_identity: ZkIdentity,
        expected_address: DepositAddress,
        autovault: bool,
    ) {
        if self.deposits.contains_key(&expected_address) {
            return; // Already registered
        }

        let deposit = ColdDeposit::new(
            detected.token,
            detected.amount,
            expected_address.clone(),
            detected.tx_hash,
            expected_identity,
            autovault,
            detected.timestamp,
        );

        self.deposits.insert(expected_address, deposit);
    }

    /// Retrieves all unclaimed deposits eligible for vault onboarding
    pub fn get_claimable(&self) -> Vec<&ColdDeposit> {
        self.deposits
            .values()
            .filter(|d| d.is_claimable())
            .collect()
    }

    /// Marks a deposit as claimed (after proof or autovault logic)
    pub fn mark_claimed(&mut self, address: &DepositAddress) {
        if let Some(deposit) = self.deposits.get_mut(address) {
            deposit.mark_claimed();
        }
    }

    /// Utility: Check if a deposit exists at given address
    pub fn exists(&self, address: &DepositAddress) -> bool {
        self.deposits.contains_key(address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::common::mock_identity_and_address;

    #[test]
    fn test_register_and_claim() {
        let (zk_id, addr) = mock_identity_and_address();
        let mut pool = ColdDepositPool::default();

        let event = DetectedDeposit {
            token: Token::BTC,
            amount: 1.25,
            tx_hash: "abc123".to_string(),
            to_address: addr.clone(),
            timestamp: Some(1699999999),
        };

        pool.register_deposit(event, zk_id.clone(), addr.clone(), true);
        assert_eq!(pool.get_claimable().len(), 1);

        pool.mark_claimed(&addr);
        assert_eq!(pool.get_claimable().len(), 0);
    }
}
