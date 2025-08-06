// ==========================================================
// common_types.rs — Shared Domex Types
// ==========================================================
//
// Defines universal structs, enums, and aliases used across
// vaults, tokens, fuel engine, validator logic, and proofs.
//

use serde::{Serialize, Deserialize};
use std::fmt::{Debug, Formatter, Result as FmtResult};

/// Vault identifier (e.g., BTC/USDT vault)
pub type VaultId = [u8; 32];

/// Poseidon commitment root (e.g., Merkle root, proof root)
pub type PoseidonRoot = [u8; 32];

/// Token type stored in vault (e.g., dBTC, dETH)
pub type TokenSymbol = String;

/// Delegator public key hash (Poseidon(vault_id + pubkey + nonce))
pub type DelegationHash = [u8; 32];

/// Global block height (used for validator attestation and sync)
pub type BlockHeight = u64;

/// Unique proof identifier (used for tracking ZK proofs)
pub type ProofId = [u8; 32];

/// User-facing fuel burn struct
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FuelBurn {
    pub vault_id: VaultId,
    pub burned_amount: u64, // In DOMEX micro-units
    pub delegated: bool,
    pub delegator: Option<[u8; 32]>, // Optional Poseidon(pubkey)
}

impl Debug for FuelBurn {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "FuelBurn {{ vault: 0x{}, amount: {}, delegated: {} }}",
            hex::encode(self.vault_id),
            self.burned_amount,
            self.delegated
        )
    }
}

/// Basic validator metadata (used in registration or attestation)
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ValidatorMeta {
    pub validator_pubkey: [u8; 32],
    pub gpu_benchmark_score: u32, // Lower is better (e.g., 24 = 24ms)
    pub stake_locked: u64,        // DOMEX staked
    pub eligible: bool,
}

/// Delegation metadata (used in fuel delegation)
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DelegationRecord {
    pub delegator_pubkey: [u8; 32],
    pub delegatee_pubkey: [u8; 32],
    pub vault_id: VaultId,
    pub nonce: u64,
    pub signature: [u8; 64], // Signature from delegator
}

/// Reward unit tracking struct (used across validator reward streams)
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ValidatorReward {
    pub validator: [u8; 32],
    pub reward_amount: u64, // In DOMEX micro-units
    pub reason: String,     // e.g., "proof_attestation", "zk_aggregate", etc.
}
