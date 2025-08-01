// Domex :: btc_vault.rs
// Core vault logic for BTC-based non-custodial Domex vaults
// Wraps identity-bound script generation, UTXO tracking, and withdrawal logic

use anyhow::{Result, anyhow};
use bitcoin::{
    Address, Network, Script, Transaction, Txid
};
use bitcoin::util::bip32::{ExtendedPrivKey, DerivationPath};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::consensus::encode::serialize;

use crate::btc_chain_client::BtcChainClient;
use crate::btc_address_utils::generate_p2wsh_address;
use crate::btc_script_template::domex_pool_script;

/// Represents a BTC-based Domex vault bound to a Poseidon identity hash
pub struct BtcVault {
    pub identity_hash: [u8; 32],
    pub vault_script: Script,
    pub vault_address: Address,
    pub chain: BtcChainClient,
    pub network: Network,

    // Optional (dev/testing): embedded key for local signing
    pub owner_xprv: Option<ExtendedPrivKey>,
}

impl BtcVault {
    /// Create a new BTC vault instance with a given identity hash and RPC credentials
    pub fn new(
        identity_hash: [u8; 32],
        network: Network,
        rpc_url: &str,
        rpc_user: &str,
        rpc_password: &str,
        owner_xprv: Option<ExtendedPrivKey>,
    ) -> Result<Self> {
        let vault_script = domex_pool_script(&identity_hash);
        let vault_address = generate_p2wsh_address(&vault_script, network);

        let chain = BtcChainClient::new(rpc_url, rpc_user, rpc_password, vault_address.clone())?;

        Ok(Self {
            identity_hash,
            vault_script,
            vault_address,
            chain,
            network,
            owner_xprv,
        })
    }

    /// Refresh UTXOs from the chain and return total balance
    pub fn sync(&mut self) -> Result<u64> {
        self.chain.refresh_utxos()?;
        Ok(self.chain.confirmed_balance())
    }

    /// Prepare a withdrawal transaction (unsigned)
    pub fn prepare_withdrawal(
        &self,
        recipient: Address,
        amount_sat: u64,
        fee_sat: u64,
    ) -> Result<Transaction> {
        self.chain.build_withdrawal_tx(recipient, amount_sat, fee_sat)
    }

    /// (Optional) Sign transaction using vault’s local private key
    /// Domex may not use this in production — ZK proof-based exit preferred
    pub fn sign_withdrawal_tx(
        &self,
        tx: &mut Transaction,
        derivation_path: &DerivationPath,
    ) -> Result<()> {
        let xprv = self.owner_xprv.as_ref()
            .ok_or_else(|| anyhow!("No local signing key available"))?;

        let secp = Secp256k1::new();
        let child_key = xprv.derive_priv(&secp, derivation_path)?;
        let privkey = child_key.private_key;

        // Placeholder: you'd use this privkey to sign each input
        // For P2WSH, signatures go into witness field (requires actual input scripts + sighash)
        // This is omitted here, since Domex uses ZK-based proof instead of signatures.

        println!("Signing TX with derived privkey: {}", privkey.to_wif());

        Ok(())
    }

    /// Broadcast a fully signed transaction
    pub fn broadcast(&self, tx: &Transaction) -> Result<Txid> {
        self.chain.broadcast_tx(tx)
    }

    /// Return the vault’s BTC address
    pub fn address(&self) -> &Address {
        &self.vault_address
    }

    /// Return the script used for locking BTC in this vault
    pub fn script(&self) -> &Script {
        &self.vault_script
    }

    /// Return the identity hash this vault is bound to
    pub fn identity(&self) -> &[u8; 32] {
        &self.identity_hash
    }
}
