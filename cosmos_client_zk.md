# Cosmos ZK Bridge Client

This module describes how the Domex protocol interacts with Cosmos-based chains using zero-knowledge proofs. It allows proof-based token onboarding and withdrawal synchronization between Cosmos and Domex.

---

## 🧠 Overview

- Users send Cosmos tokens to a monitored address
- A light client (or relayer) observes the event and generates a zk-SNARK
- The proof includes:
  - Sender Poseidon hash
  - Amount
  - Destination vault
  - Cosmos tx inclusion (via IAVL or Tendermint header)
- Domex validators verify the proof and mint corresponding dTokens

---

## 🔄 Deposit Flow

1. Cosmos user deposits token to bridge address
2. Proof generator fetches IBC tx from Cosmos node
3. ZK circuit proves:
   - Valid inclusion of tx in block
   - Correct asset amount and recipient
4. Proof is submitted to Domex validator set
5. Validators verify → dToken is minted in target vault

---

## 🔁 Withdrawal Flow

1. Vault owner burns dToken + submits exit proof
2. Domex finalizes zk + Merkle snapshot
3. Cosmos zk-client detects valid exit
4. Cosmos-side relayer releases native token

---

## 📦 zk-Circuit Components

| Element              | Description |
|----------------------|-------------|
| IBC header hash      | Proves inclusion of tx in Cosmos block |
| Sender Poseidon hash | Matches Domex vault identity           |
| Amount commitment    | Balance verified in SNARK              |
| Token ID             | Registry-verified asset                |

---

## 🔐 Security

- Cosmos tx inclusion is proven using Tendermint Merkle path
- Relayer cannot mint without zk-proof
- Replay-protected by vault ID and timestamp

---

## ✅ Circuit Hash Registration

Before onboarding any Cosmos token, its zk-verifier hash must be added to the **Domex Merkle onboarding registry** via governance.

---

> Cosmos bridges in Domex are ZK-first — no multisigs, no wrapped contracts, no trust assumptions.
