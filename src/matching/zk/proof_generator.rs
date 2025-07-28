// ===============================
// zk/proof_generator.rs : ZK Proof Builder & Submitter (Plonky2 + Pasta)
// ===============================

use crate::zk::proof_input::build_proof_input;
use crate::types::TradeResult;

use crate::zk::merkle::MerkleDelta;
use crate::types::zk::ZkProofInput;

use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::plonk::config::PoseidonGoldilocksConfig;
use plonky2::iop::target::Target;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2_field::goldilocks_field::GoldilocksField;

use plonky2::util::serialization::WriteBytes;
use std::fs::File;
use std::io::Write;

/// Main function to generate and submit a ZK proof from Raft-committed trade.
pub fn generate_and_submit_proof(trade: &TradeResult) -> Result<(), &'static str> {
    // Step 1: Convert trade into ZK-compatible struct
    let zk_input = build_proof_input(trade, trade.total_liquidity);

    // Step 2: Create a circuit builder
    let mut builder = CircuitBuilder::<GoldilocksField, PoseidonGoldilocksConfig>::new();

    // Step 3: Embed zk_input fields into the circuit manually
    embed_proof_input_into_circuit(&zk_input, &mut builder);

    // Step 4: Build and prove
    let circuit = builder.build::<PoseidonGoldilocksConfig>();
    let proof = circuit
        .prove()
        .map_err(|_| "ZK proof generation failed")?;

    // Step 5: Serialize to bytes
    let mut proof_bytes = vec![];
    proof
        .write_bytes(&mut proof_bytes)
        .map_err(|_| "Proof serialization failed")?;

    // Step 6: Submit (or save)
    submit_to_validator(&proof_bytes)?;

    Ok(())
}

/// Embeds each ZK field into the Plonky2 circuit.
/// Placeholder logic — real circuit would include constraints, hash checks, etc.
fn embed_proof_input_into_circuit(
    input: &ZkProofInput,
    builder: &mut CircuitBuilder<GoldilocksField, PoseidonGoldilocksConfig>,
) {
    //  Use descriptive variable names
    let executed_price_target = builder.constant(GoldilocksField::from_canonical_u64(input.executed_price));
    let size_target = builder.constant(GoldilocksField::from_canonical_u64(input.size));
    let liquidity_target = builder.constant(GoldilocksField::from_canonical_u64(input.total_liquidity));

    // More placeholders for buyer/seller identities (as hashes)
    let buyer_target = builder.add_virtual_target();
    let seller_target = builder.add_virtual_target();

    //  Add mock constraint for demonstration
    // builder.connect(size_target, executed_price_target); // <-- real check would go here

    // You can extend with delta balancing, ownership proofs, range proofs etc.
    println!(
        "[ZK Circuit] Embedded: price={}, size={}, liquidity={}",
        input.executed_price, input.size, input.total_liquidity
    );
}

/// Writes proof to disk (used for ETHGlobal demo — replace with validator API later)
fn submit_to_validator(proof: &[u8]) -> Result<(), &'static str> {
    let mut file =
        File::create("/tmp/domex_last_proof.bin").map_err(|_| "Failed to create proof file")?;
    file.write_all(proof)
        .map_err(|_| "Failed to write proof bytes")?;
    println!("[ZK] Proof written to disk ({} bytes)", proof.len());
    Ok(())
    }
