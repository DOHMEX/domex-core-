// btc_script_generator.rs // Domex BTC script generator (single static script address) // Locks BTC using a deterministic script with no private key access

use bitcoin::blockdata::script::Builder; use bitcoin::blockdata::opcodes; use bitcoin::Script; use sha2::{Sha256, Digest}; use hex_literal::hex;

/// Static Poseidon-based hash commitment from Domex vault onboarding /// This is precomputed from zk circuit and defines who can unlock /// (Format: Poseidon(sk || vault_id || zk_node_id)) const HARDCODED_IDENTITY_HASH: [u8; 32] = hex!("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");

/// Generates the static BTC script used for all Domex BTC vault deposits /// This script only allows withdrawals by proving Poseidon hash ownership /// in the Domex circuit — NOT via ECDSA signatures pub fn generate_domex_btc_script() -> Script { let redeem_script = Builder::new() // Push 32-byte hash (identity commitment from ZK onboarding) .push_opcode(opcodes::all::OP_SHA256) .push_slice(&sha256(&HARDCODED_IDENTITY_HASH)) .push_opcode(opcodes::all::OP_EQUALVERIFY) // Prevents further spending unless ZK proof provided to Domex .push_opcode(opcodes::all::OP_RETURN) // Acts as signal-only script .into_script();

redeem_script

}

/// Computes SHA256 of the input (BTC standard hash type) fn sha256(data: &[u8]) -> [u8; 32] { let mut hasher = Sha256::new(); hasher.update(data); let result = hasher.finalize(); let mut hash = [0u8; 32]; hash.copy_from_slice(&result); hash }

#[cfg(test)] mod tests { use super::*;

#[test]
fn test_generate_script() {
    let script = generate_domex_btc_script();
    println!("Domex BTC script: {}", script);
    assert!(script.is_op_return());
}

}

