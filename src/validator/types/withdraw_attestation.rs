// types/withdraw_attestation.rs

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithdrawAttestation {
    pub withdraw_root: String,
    pub validator_id: String,
    pub signature: String,
    pub vaults_touched: Vec<String>,
    pub total_withdrawn: u64,
    pub attested_at: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizedWithdrawPackage {
    pub withdraw_root: String,
    pub attestation: WithdrawAttestation,
}
