// ===============================
// zk/proof_generator.rs : ZK Proof Builder & Submitter
// ===============================

use crate::zk::proof_input::build_proof_input;
use crate::types::TradeResult;

use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::backend::circuit_builder::CircuitBuilder;
use plonky2::util::serialization::WriteBytes;
use pasta_curves::pallas::Point as Pallas;

use std::fs::File;
use std::io::Write;

/// Main entry to build and submit a zero-knowledge proof from a committed trade.
pub fn generate_and_submit_proof(trade: &TradeResult) -> Result<(), &'static str> {
    // Step 1: Convert trade data into a circuit-friendly input struct
    let zk_input = build_proof_input(trade);

    // Step 2: Create a Plonky2 circuit builder instance
    let mut builder = CircuitBuilder::<Pallas>::new();
    
    // Step 3: Embed all ZK input fields into the circuit
    zk_input.embed_into_circuit(&mut builder);

    // Step 4: Build and prove the circuit
    let proof = builder.build_and_prove().map_err(|_| "ZK proof generation failed")?;

    // Step 5: Serialize the proof to bytes
    let mut proof_bytes = vec![];
    proof.write_bytes(&mut proof_bytes).map_err(|_| "Proof serialization failed")?;

    // Step 6: Submit proof to validator (or save it locally for testing)
    submit_to_validator(&proof_bytes)?;

    Ok(())
}

/// Placeholder: Submits serialized proof to validator layer
fn submit_to_validator(proof: &[u8]) -> Result<(), &'static str> {
    // For testing: write proof to local disk. In production, POST to validator endpoint.
    let mut file = File::create("/tmp/domex_last_proof.bin").map_err(|_| "Failed to create file")?;
    file.write_all(proof).map_err(|_| "Failed to write proof data")?;
    println!("[ZK] Proof submitted successfully ({} bytes)", proof.len());
    Ok(())
}
