// types/proof_generator.rs
// Internal zk proof generation types for Domex onboarding

use pasta_curves::{pallas::Affine, Fp};

/// Raw secret key in 32-byte form (input for zk circuit)
#[derive(Debug, Clone)]
pub struct SecretKey {
    pub bytes: [u8; 32],
}

/// Affine public key point derived from secret key
#[derive(Debug, Clone)]
pub struct PublicKey {
    pub x: Fp,
    pub y: Fp,
}

/// Optional: Unified keypair struct for internal use
#[derive(Debug, Clone)]
pub struct Keypair {
    pub sk: SecretKey,
    pub pk: PublicKey,
}

/// Final zk-proof wrapper (can be reused for testing or bundling)
#[derive(Debug, Clone)]
pub struct ZkProof {
    pub proof_bytes: Vec<u8>,
    pub identity_hash: Fp,
}
