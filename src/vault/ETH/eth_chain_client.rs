// Domex :: eth_chain_client.rs
// ETH chain watcher for confirming vault deposits and tracking on-chain ETH balances
// Used in vault onboarding, proof generation, and Merkle updates

use ethers::prelude::*;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::poseidon_utils::poseidon_hash2;

#[derive(Debug, Clone)]
pub struct EthDepositInfo {
    pub from_address: Address,
    pub to_vault_script: Address,
    pub amount_wei: U256,
    pub block_number: U64,
    pub tx_hash: TxHash,
    pub poseidon_identity: [u8; 32],
    pub timestamp: u64,
}

pub struct EthChainClient {
    provider: Provider<Http>,
}

impl EthChainClient {
    pub fn new(rpc_url: &str) -> Self {
        let provider = Provider::<Http>::try_from(rpc_url)
            .expect("Invalid ETH RPC URL");
        Self {
            provider,
        }
    }

    /// Fetches ETH deposit events to vault script from the chain
    pub async fn fetch_deposit_to_vault(
        &self,
        vault_script_address: Address,
        from_block: u64,
        to_block: u64,
    ) -> eyre::Result<Vec<EthDepositInfo>> {
        let filter = Filter::new()
            .from_block(from_block)
            .to_block(to_block)
            .address(ValueOrArray::Value(vault_script_address));

        let logs = self.provider.get_logs(&filter).await?;

        let mut deposits = vec![];

        for log in logs {
            if let Some(tx_hash) = log.transaction_hash {
                let tx = self.provider.get_transaction(tx_hash).await?;
                if let Some(tx) = tx {
                    if tx.to == Some(vault_script_address) && tx.value > U256::zero() {
                        let poseidon_identity = poseidon_hash2(
                            &tx.from.as_bytes(),
                            &vault_script_address.as_bytes(),
                        );

                        let info = EthDepositInfo {
                            from_address: tx.from,
                            to_vault_script: vault_script_address,
                            amount_wei: tx.value,
                            block_number: tx.block_number.unwrap_or_default(),
                            tx_hash,
                            poseidon_identity,
                            timestamp: current_unix_timestamp(),
                        };

                        deposits.push(info);
                    }
                }
            }
        }

        Ok(deposits)
    }
}

/// Returns current UNIX timestamp in seconds
fn current_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
