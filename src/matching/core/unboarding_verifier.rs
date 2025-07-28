// ===============================
// core/unboarding_verifier.rs : Domex Vault Entry Validation (Phase 2)
// ===============================

use crate::types::onboarding::UserZkEntryProof;
use crate::identity::verify_poseidon_auth;
use crate::zk::merkle::verify_merkle_proof;
use crate::delta_checker::check_liquidity_delta;
use crate::vault_registry::VaultMetadata;

/// Verifies a user's ZK proof of ownership and liquidity-limited entry.
/// Activates vault locally if successful.
pub fn verify_user_entry(
    proof: &UserZkEntryProof,
    vault_meta: &VaultMetadata,
    expected_merkle_root: &str,
) -> Result<(), &'static str> {
    // Step 1: Verify Poseidon identity matches vault ID
    if !verify_poseidon_auth(&proof.poseidon_hash, &proof.vault_id) {
        return Err("Invalid Poseidon identity for vault");
    }

    // Step 2: Verify Merkle inclusion proof (ZK-proved balance)
    let verified = verify_merkle_proof(
        &proof.merkle_leaf,
        &proof.merkle_path,
        expected_merkle_root,
    );
    if !verified {
        return Err("Merkle proof verification failed");
    }

    // Step 3: Enforce 2% Delta Law based on liquidity snapshot
    if !check_liquidity_delta(proof.balance, proof.total_liquidity, vault_meta.max_delta_bps) {
        return Err("Delta violation: balance exceeds 2% of vault liquidity");
    }

    //  Passed all checks: Vault can now be activated for this user
    println!(
        "[ENTRY] User {:?} approved to enter vault {} with {} {}",
        proof.poseidon_hash, proof.vault_id, proof.balance, proof.token
    );

    Ok(())
}
