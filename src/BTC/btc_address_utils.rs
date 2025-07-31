// Domex :: btc_address_utils.rs
// Utility functions for handling script-based BTC addresses used in Domex pool design

use bitcoin::blockdata::script::Script;
use bitcoin::util::address::Address;
use bitcoin::network::constants::Network;
use bitcoin::util::schnorr::SchnorrPubkey;

/// Generates a BTC P2WSH (Pay-to-Witness-Script-Hash) address from the locking script.
/// This is used to derive the Domex pool address from its predefined script logic.
pub fn generate_p2wsh_address(script: &Script, network: Network) -> Address {
    Address::p2wsh(script, network)
}

/// Optionally derive a Taproot address (if using Schnorr script path control)
/// This assumes a single internal key model and no control block path.
/// Not used in core Domex model but included for completeness.
pub fn generate_taproot_address(pubkey: &SchnorrPubkey, network: Network) -> Address {
    Address::p2tr(network, pubkey, None)
}

/// Verifies whether a given BTC address corresponds to a known Domex pool script.
/// Can be used during bridge deposit confirmation checks.
pub fn is_domex_pool_address(address: &Address, expected_script: &Script) -> bool {
    match address.payload.script_pubkey() {
        s if &s == expected_script => true,
        _ => false,
    }
}
