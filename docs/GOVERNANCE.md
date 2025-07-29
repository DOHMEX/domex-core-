# Domex Governance: Token Onboarding & Gatekeeper DAO

To preserve the integrity of the Domex vault system, every token made liquid on the shared settlement layer must pass through a strict ZK-governed onboarding process. This ensures only valid, verifiable, and cryptographically provable assets participate in trading.

---

## 🧱 Governance Model

Domex uses a modular, ZK-auditable onboarding process governed by the **Gatekeeper DAO**. This DAO enforces technical and cryptoeconomic standards before any asset can be linked to a vault.

---

## 🔄 Token Onboarding Lifecycle

### 1. Asset Proposal

A proposer (individual, protocol, DAO, or zk-client author) submits an onboarding request that includes:

- Native chain (e.g., Bitcoin, Ethereum, Cosmos, etc.)
- Proof generation flow (Plonky2 only)
- ZK circuit hash (vault-bound onboarding circuit)
- Intended economic role (e.g., base pair, stablecoin, synthetic)
- Liquidity estimate and use case description

---

### 2. ZK Circuit Review

The Gatekeeper DAO committee, supported by technical auditors, reviews each submission for:

- Valid Plonky2 circuit architecture
- Correct proof-of-origin (e.g., BTC transfer inclusion)
- Poseidon-bound identity compliance
- Compatibility with Domex’s vault and state root model

✅ If accepted, the circuit verifier hash is added to the **global Merkle registry** of allowed onboarding circuits.

---

### 3. DAO Voting

The full Gatekeeper DAO votes on the asset proposal. A supermajority is required to activate the token:

- Vote includes risk assessment, circuit audit hash, and vault classification
- Only tokens passing both circuit review and DAO governance can be assigned vault liquidity

---

## ✅ Summary

| Component       | Requirement                          |
|----------------|---------------------------------------|
| Circuit Type    | **Plonky2 ZK-SNARK only**            |
| Custody Method  | Poseidon-bound vault onboarding      |
| Bridge Required | ❌ No (Domex does not use bridges)   |
| Token Whitelist | Maintained via Merkle registry       |
| Governance Body | Gatekeeper DAO                       |

Only tokens that pass **ZK-level validation** and **DAO-level approval** become active on Domex.
