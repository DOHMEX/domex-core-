/// Builds a raw Bitcoin withdrawal transaction to multiple recipients
/// using selected UTXOs from a Domex vault (supports batching).
///
/// # Arguments
/// - `utxos`: Available unspent outputs from the vault
/// - `recipients`: Vec of (recipient_address, amount_sat) tuples
/// - `change_address`: Address to return remaining change (must be vault-controlled)
/// - `fee_sat`: Network fee in satoshis
///
/// # Returns
/// Unsigned Bitcoin transaction to be signed and broadcast
pub fn build_btc_withdrawal_tx_batched(
    utxos: &[VaultUtxo],
    recipients: Vec<(Address, u64)>,
    change_address: &Address,
    fee_sat: u64,
) -> Result<Transaction, String> {
    // 1. Compute total output amount
    let total_output: u64 = recipients.iter().map(|(_, amt)| *amt).sum();
    let required_total = total_output + fee_sat;

    // 2. Select UTXOs that cover the required amount
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
        return Err("Insufficient funds in vault for batched withdrawal".into());
    }

    // 3. Construct transaction inputs
    let inputs: Vec<TxIn> = selected
        .iter()
        .map(|utxo| TxIn {
            previous_output: OutPoint {
                txid: utxo.txid,
                vout: utxo.vout,
            },
            script_sig: Script::new(), // P2WSH
            sequence: 0xffffffff,
            witness: vec![],           // Filled later with ZK proof
        })
        .collect();

    // 4. Construct outputs: recipients
    let mut outputs: Vec<TxOut> = recipients
        .iter()
        .map(|(addr, amt)| TxOut {
            value: *amt,
            script_pubkey: addr.script_pubkey(),
        })
        .collect();

    // 5. Add change output if needed
    let change = total_input.saturating_sub(required_total);
    if change >= 546 {
        outputs.push(TxOut {
            value: change,
            script_pubkey: change_address.script_pubkey(),
        });
    }

    // 6. Final transaction
    let tx = Transaction {
        version: 2,
        lock_time: 0,
        input: inputs,
        output: outputs,
    };

    Ok(tx)
}
