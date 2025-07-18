# Domex Protocol Overview

Domex is a ZK-native, transactionless blockchain protocol designed to unify global liquidity across chains. It replaces traditional transaction execution with zero-knowledge proof-based state changes, enabling verifiable, gasless, and quantum-safe market coordination.

---

## 🌍 Problem

Current DeFi and CEX ecosystems are fragmented:
- Liquidity is siloed across blockchains, apps, and order books.
- Execution relies on wallets, mempools, and on-chain transactions.
- Settlement is trust-based or opaque, especially across chains.

---

## 🔑 Domex Solution

Domex introduces a **vault-state execution model** where all market actions are triggered by verifiable zero-knowledge proofs — no transactions, no wallets, no mempools.

All trade settlement, custody transitions, and withdrawals are cryptographically enforced via Poseidon-hashed identities and zk-SNARK validation.

---

## 🧱 Core Architecture

### 1. Vault-State Execution (Phase 1)
- Assets (e.g., BTC, ETH) are bridged into Domex via zk-proof clients.
- dTokens (e.g., dBTC) are minted and credited to the sender’s vault.
- Vault ownership is tied to Poseidon-hashed identity — not a wallet address.

### 2. Trading Intent & Matching (Phase 2)
- Users submit trade intents via JSON-based order schema.
- Matching happens within vaults via local Raft-style consensus (sub-second speed).
- Once matched, claim rights are updated and vault ownership changes.

### 3. ZK-Proof Withdrawal (Phase 3)
- Vault owners can generate ZK proofs to burn dTokens and reclaim native assets.
- Proof is submitted, validated against Merkle/Verkle root.
- Validators finalize the withdrawal and trigger bridge execution.

---

## 🧠 Key Properties

| Feature                     | Description |
|----------------------------|-------------|
| **Transactionless**        | All state changes happen via zk-proofs, not txs. |
| **Walletless**             | Identity = Poseidon hash, not keypairs. |
| **No Mempool**             | Orders are not broadcast; matched locally and finalized via zk. |
| **Quantum-Resistant**      | No ECDSA; all identity logic uses Poseidon. |
| **Modular Vault Logic**    | Every market has its own Raft-based vault engine. |
| **ZK Withdrawals**         | All exits require zk-proof-based burn + exit flow. |
| **Global Merkle Finality** | Validators finalize Merkle root snapshots of all state changes. |

---

## 🔗 Cross-Chain Design

Domex supports native ZK clients for bridging:

- **Cosmos** (live)
- **Ethereum** (in development)
- **Solana** (planned)
- Other chains via client-specific zk circuits

Each bridge uses zk-proofs to verify incoming deposits and process exit proofs.

---

## 🏛️ Governance & Onboarding

- New tokens must be onboarded via DAO vote and registry inclusion.
- Gatekeeper DAO reviews bridge ZK circuits and economic risk.
- All onboarding logic is tied to a ZK-auditable registry.

---

## 🛡️ Validator DAO & Slashing

- Validators finalize only zk-verified Merkle snapshots.
- Slashing conditions are enforced via proof failure or consensus deviation.
- Rewards are streamed based on zk-proof verification volume.

---

## 🔄 Data & APIs

- Trade intent format: `schema/order_format.json`
- Proof payload format: `schema/proof_payload.json`
- ZK circuit design: `zk/circuit_overview.md`

---

## 🧪 Road to Mainnet

- ✅ Architecture finalized
- ✅ Vault custody + zk execution integrated
- 🔄 Multi-chain zk bridge in progress
- 🔄 Shared vault interface (MVP)
- 🔜 Validator onboarding + zk audit tools
- 🔜 Public testnet (Q4 2025)

---

## 📝 Related Files

- Vault Execution: `core/vault_logic.md`
- Poseidon Identity: `core/identity_poseidon.md`
- Merkle/ZK Flow: `core/merkle_flow_diagram.png`
- Bridges: `bridge/`
- ZK Circuits: `zk/`

---

Domex is the first protocol to offer **verifiable global liquidity coordination** — trustless, modular, and ZK-native by design.
