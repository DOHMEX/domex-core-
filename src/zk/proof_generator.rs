// ===============================
// zk/proof_generator.rs : ZK Proof Builder & Submitter
// ===============================

use crate::zk::proof_input::build_zk_input;
use crate::types::TradeResult;

use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::backend::circuit_builder::CircuitBuilder;
use plonky2::util::serialization::WriteBytes;
use pasta_curves::pallas::Point as Pallas;

use std::fs::File;
use std::io::Write;

/// Main entry to build + submit a ZK proof based on a committed trade.
pub fn generate_and_submit_proof(trade: &TradeResult) -> Result<(), &'static str> {
    // Step 1: Build ZK circuit inputs from trade
    let input_data = build_zk_input(trade)?;

    // Step 2: Construct Plonky2 circuit
    let mut builder = CircuitBuilder::<Pallas>::new();
    input_data.embed_into_circuit(&mut builder);

    // Step 3: Generate the proof
    let proof = builder.build_and_prove().map_err(|_| "Proof generation failed")?;

    // Step 4: Serialize proof
    let mut proof_bytes = vec![];
    proof.write_bytes(&mut proof_bytes).map_err(|_| "Proof serialization failed")?;

    // Step 5: Submit to global validator layer (placeholder endpoint)
    submit_to_validator(&proof_bytes)?;

    Ok(())
}

/// Mock function to simulate validator submission
fn submit_to_validator(proof: &[u8]) -> Result<(), &'static str> {
    // For now, just dump to disk (or send to HTTP endpoint in real build)
    let mut file = File::create("/tmp/domex_last_proof.bin").map_err(|_| "Write fail")?;
    file.write_all(proof).map_err(|_| "Write error")?;
    println!("[ZKP] Proof submitted: {} bytes", proof.len());
    Ok(())
}
