// ===============================
// zk/proof_generator.rs — Domex ZK Proof Builder & Submitter (Ponkey2 + Pasta)
// ===============================

use crate::types::TradeResult;
use crate::zk::proof_input::build_proof_input;
use crate::types::zk::ZkProofInput;
use crate::zk::merkle::MerkleDelta;
use crate::zk::ponkey2_backend::generate_ponkey2_proof;

use std::fs::File;
use std::io::Write;

/// Generates a Ponkey2-compatible ZK proof for a committed trade result.
/// This is triggered after Raft consensus by a local node.
pub fn generate_and_submit_proof(trade: &TradeResult) -> Result<(), &'static str> {
    let zk_input: ZkProofInput = build_proof_input(trade)?;

    // Build the ZK proof using Domex-native Ponkey2 backend
    let proof_bytes = generate_ponkey2_proof(&zk_input)
        .map_err(|_| "Failed to generate Ponkey2 proof")?;

    // Submit proof to validator (for now writes to disk — production uses broadcast)
    submit_to_validator(&proof_bytes)?;

    Ok(())
}

/// Writes the ZK proof bytes to disk for demo/testing (replace with validator API later)
fn submit_to_validator(proof: &[u8]) -> Result<(), &'static str> {
    let mut file =
        File::create("/tmp/domex_last_proof.bin").map_err(|_| "Failed to create proof file")?;
    file.write_all(proof)
        .map_err(|_| "Failed to write proof bytes")?;
    println!("[ZK] Proof written to disk ({} bytes)", proof.len());
    Ok(())
}
