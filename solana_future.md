# Solana ZK Bridge (Planned Integration)

This document outlines the design intent for bridging Solana-based assets to and from Domex using a future ZK client. While not yet implemented, this bridge will follow the same trust-minimized principles as the Cosmos and Ethereum clients.

---

## 🧭 Goal

To enable users to:

- Onboard SPL tokens (e.g., SOL, USDC) from Solana to Domex via zk-proof
- Withdraw from Domex vaults back to Solana using zk burn proof
- Avoid using wrapped assets, relayers, or trusted bridges

---

## 🔄 Planned Deposit Flow (SPL Token → Domex)

1. User sends SPL token to a known program-derived address (PDA) on Solana
2. zk-proof generator confirms:
   - Inclusion in Solana ledger (via state root or proof of account data)
   - Correct source address + destination Poseidon ID
   - Token amount and type
3. zk-proof is submitted to Domex
4. Validators verify → dToken (e.g., dSOL) is minted in specified vault

---

## 🔁 Planned Withdrawal Flow (Domex → Solana)

1. Vault owner generates zk-proof of:
   - Vault ownership (Poseidon ID)
   - Burned amount
   - Destination Solana wallet
2. Domex validators finalize Merkle root containing burn
3. Solana zk-client verifies exit proof and releases SPL token from PDA

---

## 🧩 Circuit Requirements

- Solana account proof inclusion
- PDA-bound deposit validation
- Poseidon-based user identity linking
- Support for Solana block hash verification

---

## ⚙️ Engineering Dependencies

- Solana zk-compatible ledger proof system (e.g., proof of account state)
- Light-client verifier integration or trusted witness fallback (temporarily)
- Custom SNARK for Solana ledger format

---

## 📝 Registry Note

The Solana zk bridge verifier must be audited and added to the Domex onboarding Merkle registry before production deployment.

---

## ❗ Status

This module is currently under research. Testnet deployment is targeted for mid-to-late 2026.

---

> Solana bridging in Domex will preserve full trustlessness — with no wrapped tokens, no intermediaries, and no signature reliance.
