// Domex :: btc_withdraw_builder.rs
// Constructs raw Bitcoin transactions from Domex vault UTXOs for withdrawal execution
// Uses standard P2WSH format and Poseidon-locked vault scripts

use bitcoin::blockdata::transaction::{Transaction, TxIn, TxOut, OutPoint};
use bitcoin::blockdata::script::Script;
use bitcoin::util::amount::Amount;
use bitcoin::consensus::encode::serialize;
use bitcoin::Network;
use bitcoin::Address;
use bitcoin::Txid;

/// Represents a single unspent BTC vault output
#[derive(Debug, Clone)]
pub struct VaultUtxo {
    pub txid: Txid,
    pub vout: u32,
    pub value_sat: u64,
    pub script_pubkey: Script,
}

/// Builds a raw Bitcoin withdrawal transaction for the given recipient
/// using selected UTXOs from a Domex vault
///
/// # Arguments
/// - `utxos`: Available unspent outputs from the vault
/// - `recipient`: BTC address to receive the funds
/// - `amount_sat`: Amount to send in satoshis
/// - `change_address`: Address to return remaining change (must be vault-controlled)
/// - `fee_sat`: Network fee in satoshis
///
/// # Returns
/// Unsigned Bitcoin transaction to be signed and broadcast
pub fn build_btc_withdrawal_tx(
    utxos: &[VaultUtxo],
    recipient: &Address,
    amount_sat: u64,
    change_address: &Address,
    fee_sat: u64,
) -> Result<Transaction, String> {
    // 1. Select UTXOs that can cover the withdrawal + fee
    let mut selected = vec![];
    let mut total = 0u64;

    for utxo in utxos {
        selected.push(utxo.clone());
        total += utxo.value_sat;
        if total >= amount_sat + fee_sat {
            break;
        }
    }

    if total < amount_sat + fee_sat {
        return Err("Insufficient funds in vault".into());
    }

    // 2. Construct inputs
    let inputs: Vec<TxIn> = selected
        .iter()
        .map(|utxo| TxIn {
            previous_output: OutPoint {
                txid: utxo.txid,
                vout: utxo.vout,
            },
            script_sig: Script::new(), // P2WSH: empty
            sequence: 0xffffffff,
            witness: vec![],           // Will be filled by the off-chain proof system
        })
        .collect();

    // 3. Construct outputs (recipient + optional change)
    let mut outputs = vec![TxOut {
        value: amount_sat,
        script_pubkey: recipient.script_pubkey(),
    }];

    let change = total.saturating_sub(amount_sat + fee_sat);
    if change >= 546 {
        outputs.push(TxOut {
            value: change,
            script_pubkey: change_address.script_pubkey(),
        });
    }

    // 4. Build the unsigned transaction
    let tx = Transaction {
        version: 2,
        lock_time: 0,
        input: inputs,
        output: outputs,
    };

    Ok(tx)
}
