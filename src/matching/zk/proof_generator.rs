// ===============================
// zk/proof_generator.rs — Domex ZK Proof Builder & Submitter (Ponkey2: Plonky2 + Poseidon)
// ===============================

use crate::types::TradeResult;
use crate::zk::proof_input::build_proof_input;
use crate::types::zk::ZkProofInput;
use crate::zk::merkle::MerkleDelta;
use crate::zk::plonky2_backend::generate_plonky2_proof;

use std::fs::File;
use std::io::Write;

/// Generates a Plonky2-based ZK proof for a finalized trade result.
/// This is triggered after Raft consensus by a local node and submitted to global validators.
pub fn generate_and_submit_proof(trade: &TradeResult) -> Result<(), &'static str> {
    // Step 1: Build ZK-compatible input from TradeResult (Merkle delta, vault info, etc.)
    let zk_input: ZkProofInput = build_proof_input(trade)?;

    // Step 2: Generate ZK proof using Ponkey2 = Plonky2 circuit backend
    let proof_bytes = generate_plonky2_proof(&zk_input)
        .map_err(|_| "Failed to generate Plonky2 proof")?;

    // Step 3: Submit proof (demo: write to disk; production: network broadcast to validators)
    submit_to_validator(&proof_bytes)?;

    Ok(())
}

/// Writes the ZK proof bytes to disk for testing (temporary stub)
fn submit_to_validator(proof: &[u8]) -> Result<(), &'static str> {
    let mut file = File::create("/tmp/domex_last_proof.bin")
        .map_err(|_| "Failed to create proof file")?;

    file.write_all(proof)
        .map_err(|_| "Failed to write proof bytes")?;

    println!("[ZK]  Plonky2 proof written to disk ({} bytes)", proof.len());
    Ok(())
}
