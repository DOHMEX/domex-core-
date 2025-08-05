// Domex :: btc_withdraw_builder.rs
// Constructs raw Bitcoin transactions from Domex vault UTXOs for withdrawal execution
// Supports batching multiple recipients securely

use bitcoin::blockdata::transaction::{Transaction, TxIn, TxOut, OutPoint};
use bitcoin::blockdata::script::Script;
use bitcoin::{Address, Txid};

/// Represents a single unspent BTC vault output
#[derive(Debug, Clone)]
pub struct VaultUtxo {
    pub txid: Txid,
    pub vout: u32,
    pub value_sat: u64,
    pub script_pubkey: Script,
}

/// Builds a raw Bitcoin withdrawal transaction to multiple recipients
///
/// # Arguments
/// - `utxos`: Vault UTXOs to spend from
/// - `recipients`: Vector of (BTC address, satoshis) to send
/// - `change_address`: Vault-controlled BTC address to receive change
/// - `fee_sat`: Network fee in satoshis
///
/// # Returns
/// Unsigned Bitcoin transaction for ZK proof + signature
pub fn build_btc_withdrawal_tx(
    utxos: &[VaultUtxo],
    recipients: Vec<(Address, u64)>,
    change_address: &Address,
    fee_sat: u64,
) -> Result<Transaction, String> {
    let total_output: u64 = recipients.iter().map(|(_, amt)| *amt).sum();
    let required_total = total_output.checked_add(fee_sat).ok_or("Overflow")?;

    // Select UTXOs to cover total
    let mut selected = vec![];
    let mut total_input = 0u64;

    for utxo in utxos {
        selected.push(utxo.clone());
        total_input += utxo.value_sat;
        if total_input >= required_total {
            break;
        }
    }

    if total_input < required_total {
        return Err("Insufficient vault funds".into());
    }

    // Build inputs
    let inputs: Vec<TxIn> = selected
        .iter()
        .map(|u| TxIn {
            previous_output: OutPoint {
                txid: u.txid,
                vout: u.vout,
            },
            script_sig: Script::new(),
            sequence: 0xffffffff,
            witness: vec![],
        })
        .collect();

    // Build outputs
    let mut outputs: Vec<TxOut> = recipients
        .into_iter()
        .map(|(addr, amt)| TxOut {
            value: amt,
            script_pubkey: addr.script_pubkey(),
        })
        .collect();

    // Optional change
    let change = total_input - required_total;
    if change >= 546 {
        outputs.push(TxOut {
            value: change,
            script_pubkey: change_address.script_pubkey(),
        });
    }

    Ok(Transaction {
        version: 2,
        lock_time: 0,
        input: inputs,
        output: outputs,
    })
}
