// types/deposit_address.rs
// Typed structs for EC public key and deposit address logic in Domex zk onboarding

use pasta_curves::Fp;

/// Structured public key (x, y) in Pasta field
#[derive(Debug, Clone)]
pub struct PublicKey {
    pub x: Fp,
    pub y: Fp,
}

/// Optional: Native chain address derived from public key (if client uses this mapping)
#[derive(Debug, Clone)]
pub struct DepositAddress {
    pub chain: String,       // e.g., "BTC", "ETH", "SOL"
    pub address: String,     // e.g., base58 or bech32 native address
}
