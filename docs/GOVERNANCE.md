# Domex Governance: Token Onboarding & Gatekeeper DAO (Step 8)

To preserve the integrity of the Domex trading layer, every token onboarded into the vault system must pass through a governance and verification flow. This ensures only valid, auditable assets are made liquid on the shared order book.

---

## 🧱 Governance Model

Domex uses a modular, zk-auditable onboarding process governed by the Gatekeeper DAO.

---

## 🔄 Token Onboarding Lifecycle

### 1. Asset Proposal

A proposer (individual, protocol, DAO, or bridge partner) submits a token onboarding request.

**Required metadata:**
- Native chain (e.g., BTC, ETH, Cosmos)
- Bridge proof format (e.g., Groth16, PLONK)
- ZK circuit hash
- Economic use case and liquidity estimate

### 2. ZK Circuit Review

Submitted bridge circuits are verified for:
- Correct proof-of-origin (e.g., BTC deposit)
- Poseidon-bound identity mapping
- Compatibility with vault execution model  
- Verifier hash is added to the Merkle registry

### 3. DAO Voting

The Gatekeeper DAO reviews proposals and votes on token onboarding status. Only tokens passing technical review and community governance are accepted into the active vault set.

---
