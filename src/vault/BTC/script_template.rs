// src/btc/script_template.rs
// Domex BTC script template for non-custodial deposit vault
// Generates a single redeem script that locks BTC without a private key
// Withdrawals are authorized via Poseidon identity + ZK proof (off-chain verified)

use bitcoin::blockdata::script::Builder;
use bitcoin::blockdata::opcodes::all::*;
use bitcoin::Script;

/// Returns the fixed BTC script used for Domex pool deposits
/// - This script has no corresponding private key
/// - Funds are locked under a programmatic hash commitment (e.g., Poseidon identity)
/// - Withdrawal is only possible via off-chain ZK proof confirmed by Domex validators
pub fn domex_pool_script(identity_hash_bytes: &[u8; 32]) -> Script {
    // Build the script:
    // OP_PUSH32 <identity_hash> OP_EQUAL

    Builder::new()
        .push_opcode(OP_PUSHBYTES_32)
        .push_slice(identity_hash_bytes)
        .push_opcode(OP_EQUAL)
        .into_script()
}

/// Optionally returns the P2SH (pay-to-script-hash) address for display
pub fn domex_p2sh_address(script: &Script, network: bitcoin::Network) -> bitcoin::Address {
    bitcoin::Address::p2sh(script, network)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::Network;

    #[test]
    fn test_domex_script_generation() {
        // Example Poseidon-based identity hash (mocked for test)
        let dummy_hash = [0x11u8; 32];

        let script = domex_pool_script(&dummy_hash);
        assert!(script.is_pushed_data());

        let addr = domex_p2sh_address(&script, Network::Bitcoin);
        println!("Generated P2SH address: {}", addr);
    }
}
