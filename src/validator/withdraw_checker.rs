// validator/withdraw_checker.rs // Domex validator-side withdrawal verification logic // Ensures withdrawal intent matches Merkle state and ZK proof validity

use crate::types::{ withdrawal::WithdrawRequest, vault::VaultState, poseidon_utils::verify_withdrawal_hash, circuit_interface::{ZkWithdrawalProof, ZkProverError}, }; use crate::validator::zk_verifier::verify_withdrawal_proof;

/// Verifies a withdrawal request against current Merkle-encoded vault state /// Ensures: /// - ZK proof is valid /// - Withdrawal hash matches expected identity balance /// - Withdraw amount does not exceed vault balance pub fn check_withdrawal_request( request: &WithdrawRequest, vault_state: &VaultState, ) -> Result<(), ZkProverError> { let identity = &request.identity_hash;

// 1. Check vault balance exists
let balance = vault_state
    .balances
    .get(identity)
    .ok_or(ZkProverError::UnknownVault)?;

// 2. Check withdrawal amount is valid
if request.amount == 0 || request.amount > *balance {
    return Err(ZkProverError::InvalidWithdrawalAmount);
}

// 3. Recompute withdrawal hash and verify intent
let valid_hash = verify_withdrawal_hash(
    &request.withdrawal_hash,
    identity,
    request.amount,
    request.timestamp,
);
if !valid_hash {
    return Err(ZkProverError::WithdrawalHashMismatch);
}

// 4. Verify withdrawal ZK proof
verify_withdrawal_proof(&request.zk_proof, request)?

}

