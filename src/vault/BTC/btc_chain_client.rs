// Domex :: btc_chain_client.rs
// Real BTC chain client to track vault address and build withdrawals

use bitcoin::{
    Address, Network, Script, Transaction, TxIn, TxOut, Txid, OutPoint,
    consensus::encode::serialize,
};
use bitcoincore_rpc::{Auth, Client, RpcApi};
use anyhow::{Result, anyhow};
use std::collections::HashMap;

/// Chain-level BTC vault manager
pub struct BtcChainClient {
    rpc: Client,
    vault_address: Address,
    utxos: HashMap<(Txid, u32), TxOut>,
}

impl BtcChainClient {
    /// Create a new client connected to a Bitcoin node
    pub fn new(
        rpc_url: &str,
        rpc_user: &str,
        rpc_password: &str,
        vault_address: Address,
    ) -> Result<Self> {
        let rpc = Client::new(rpc_url, Auth::UserPass(rpc_user.into(), rpc_password.into()))?;
        Ok(Self {
            rpc,
            vault_address,
            utxos: HashMap::new(),
        })
    }

    /// Refresh UTXOs associated with this vault address (confirmed only)
    pub fn refresh_utxos(&mut self) -> Result<()> {
        let raw_addr = self.vault_address.to_string();
        let list = self.rpc.list_unspent(Some(1), None, Some(&[raw_addr]), None, None)?;

        self.utxos.clear();

        for entry in list {
            let tx_out = TxOut {
                value: entry.amount.as_sat(),
                script_pubkey: self.vault_address.script_pubkey(),
            };
            self.utxos.insert((entry.txid, entry.vout), tx_out);
        }

        Ok(())
    }

    /// Return the total balance in satoshis (confirmed only)
    pub fn confirmed_balance(&self) -> u64 {
        self.utxos.values().map(|txo| txo.value).sum()
    }

    /// Build a raw unsigned BTC transaction to withdraw from vault
    /// Does not handle fee estimation — fee must be manually provided
    pub fn build_withdrawal_tx(
        &self,
        recipient: Address,
        amount_sat: u64,
        fee_sat: u64,
    ) -> Result<Transaction> {
        let mut inputs = Vec::new();
        let mut collected = 0;

        for ((txid, vout), tx_out) in &self.utxos {
            inputs.push(TxIn {
                previous_output: OutPoint {
                    txid: *txid,
                    vout: *vout,
                },
                script_sig: Script::new(),
                sequence: 0xffffffff,
                witness: vec![],
            });
            collected += tx_out.value;
            if collected >= amount_sat + fee_sat {
                break;
            }
        }

        if collected < amount_sat + fee_sat {
            return Err(anyhow!(
                "Insufficient funds: need {} sat, have {} sat",
                amount_sat + fee_sat,
                collected
            ));
        }

        let mut outputs = vec![TxOut {
            value: amount_sat,
            script_pubkey: recipient.script_pubkey(),
        }];

        if collected > amount_sat + fee_sat {
            // Return change to vault
            outputs.push(TxOut {
                value: collected - amount_sat - fee_sat,
                script_pubkey: self.vault_address.script_pubkey(),
            });
        }

        Ok(Transaction {
            version: 2,
            lock_time: 0,
            input: inputs,
            output: outputs,
        })
    }

    /// Broadcast a signed transaction to the BTC network
    pub fn broadcast_tx(&self, tx: &Transaction) -> Result<Txid> {
        let txid = self.rpc.send_raw_transaction(tx)?;
        Ok(txid)
    }

    /// Return UTXO count (useful for diagnostics)
    pub fn utxo_count(&self) -> usize {
        self.utxos.len()
    }

    /// Return the raw unsigned TX hex (for external signing tools)
    pub fn get_raw_tx_hex(&self, tx: &Transaction) -> String {
        hex::encode(serialize(tx))
    }
}
