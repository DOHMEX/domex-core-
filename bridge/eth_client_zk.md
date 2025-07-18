# Ethereum ZK Bridge Client

Domex connects to Ethereum through a **ZK client bridge**, enabling trustless onboarding and withdrawals of native ERC-20 assets.

All transfers rely on client-generated zk-proofs, not custody contracts or relayers.

---

## 🧠 Overview

- Deposits from Ethereum must be proven with zk-SNARKs showing valid inclusion in Ethereum state
- Exits are finalized only after a Domex burn + proof verification
- No Domex contracts are deployed on Ethereum, only ZK bridges and event watchers

---

## 🔄 Deposit Flow (ETH → Domex)

1. User sends ETH or ERC-20 token to a monitored Ethereum address
2. zk-proof generator verifies:
   - Inclusion of tx in Ethereum block
   - Sender address and vault-binding data
   - Token type and amount
3. Proof is submitted to Domex
4. Validators verify and mint dToken (e.g., dETH, dUSDC)

---

## 🔁 Withdrawal Flow (Domex → ETH)

1. Vault owner burns dToken (e.g., dUSDC)
2. Submits zk-proof with:
   - Vault ID
   - Poseidon identity
   - Burn amount and destination ETH address
3. Validators verify and finalize Merkle exit
4. Ethereum zk-client detects burn and unlocks native ERC-20

---

## 🔐 zk Circuit Proof Elements

| Field             | Description |
|------------------|-------------|
| Ethereum block header hash | Validates origin block |
| tx index + hash   | Proves inclusion of transaction |
| Sender Poseidon ID| Connects Ethereum tx to Domex identity |
| ERC-20 proof      | Confirms token + amount         |
| ZK commitment hash| Prevents replay or duplication |

---

## 🧱 Design Principles

- **No wrapped assets**  
- **No multisigs or relayers**  
- **Exit validation is purely zk-based**  
- **Supports batch deposits and exits**

---

## 🧠 Optional Enhancements

- Use zk-rollup snapshot of exit ledger on Ethereum
- Add optimistic fallback with fraud-proof timer (for L2s)

---

## 📝 Circuit Registry Requirement

Every Ethereum token bridge circuit must be verified and whitelisted in the **Merkle onboarding registry**, managed by the Gatekeeper DAO.

---

> The Ethereum bridge client enables gasless, verifiable, zk-controlled movement of assets — without compromising trust.
