// ==========================================================
// btc_redeem_checker.rs — Domex BTC Redeem Verifier
// ==========================================================
//
// Checks whether a BTC withdrawal tx has been redeemed.
// Used by vaults to finalize user exit from Domex.

use crate::btc_chain_client::get_tx_by_hash;
use crate::vault::BTC::btc_address_utils::address_to_script_pubkey;
use bitcoin::{Transaction, TxOut};
use std::time::Duration;

/// Represents a successful BTC redemption check
#[derive(Debug)]
pub struct RedeemStatus {
    pub confirmed: bool,
    pub block_height: Option<u64>,
    pub output_found: bool,
}

/// Checks if a BTC tx hash includes the expected recipient address and amount
pub fn check_btc_redeem(
    tx_hash: &str,
    expected_address: &str,
    expected_amount: u64, // in satoshis
) -> RedeemStatus {
    // === Fetch raw transaction ===
    let tx: Option<Transaction> = get_tx_by_hash(tx_hash, Some(Duration::from_secs(10)));

    if let Some(tx) = tx {
        // === Convert expected address to scriptPubKey ===
        let expected_script = match address_to_script_pubkey(expected_address) {
            Ok(script) => script,
            Err(_) => return RedeemStatus {
                confirmed: false,
                block_height: None,
                output_found: false,
            },
        };

        // === Check if expected output exists ===
        let output_found = tx.output.iter().any(|TxOut { value, script_pubkey }| {
            *value == expected_amount && *script_pubkey == expected_script
        });

        // === Assume confirmation is done off-chain (can check block height) ===
        RedeemStatus {
            confirmed: true, // optionally make this conditional on block confirmations
            block_height: Some(820000), // placeholder
            output_found,
        }
    } else {
        RedeemStatus {
            confirmed: false,
            block_height: None,
            output_found: false,
        }
    }
}
