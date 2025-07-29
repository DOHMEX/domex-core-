// deposit_checker.rs
// Verifies that a user's zk proof corresponds to a valid native deposit
// Part of Domex zk onboarding flow

use crate::types::zk_client::ZkOnboardingPublicInputs;
use crate::types::proof_generator::PublicKey;
use crate::types::deposit_status::DepositStatus;
use crate::types::deposit_asset::ChainAssetType;
use crate::deposit_address::derive_address_for_chain;

/// Verifies that the user made a valid deposit to their derived address
/// Assumes the public key (pk_x, pk_y) came from zk circuit
pub fn verify_native_deposit(
    inputs: &ZkOnboardingPublicInputs,
    asset: ChainAssetType,
    fetch_tx_status: impl Fn(&str) -> Option<DepositStatus>,
) -> Result<(), DepositVerificationError> {
    // Step 1: Reconstruct public key
    let user_pk = PublicKey {
        x: inputs.pk_x,
        y: inputs.pk_y,
    };

    // Step 2: Derive expected deposit address (e.g., BTC address)
    let expected_address = derive_address_for_chain(&user_pk, &asset)?;

    // Step 3: Fetch deposit transaction status
    let status = fetch_tx_status(&inputs.deposit_tx_hash)
        .ok_or(DepositVerificationError::TxNotFound)?;

    // Step 4: Match address in transaction
    if status.to_address != expected_address {
        return Err(DepositVerificationError::AddressMismatch {
            expected: expected_address,
            found: status.to_address,
        });
    }

    // Step 5: Check minimum deposit amount
    if status.amount < asset.min_onboard_amount() {
        return Err(DepositVerificationError::AmountTooLow {
            required: asset.min_onboard_amount(),
            found: status.amount,
        });
    }

    Ok(())
}

/// Errors during deposit validation
#[derive(Debug, Clone)]
pub enum DepositVerificationError {
    TxNotFound,
    AddressMismatch {
        expected: String,
        found: String,
    },
    AmountTooLow {
        required: u64,
        found: u64,
    },
    AddressDerivationFailed,
}
