# Domex Protocol Roadmap

Domex is building a ZK-native execution layer to unify global liquidity. This roadmap outlines our phased development approach from testnet to validator decentralization.

---

## ✅ Completed (Q1–Q2 2025)

- ✅ Vault-state execution model finalized  
- ✅ ZK Key for direct receipt and Poseidon identity system implemented  
- ✅ Local vault consensus (Raft) logic stable  
- ✅ Global validator ZK-finality model locked  
- ✅ Trade intent schema + matching API defined  
- ✅ ZK withdrawal logic fully designed  
- ✅ Governance framework for token inflow approved  

---

## 🔄 In Progress (Q3 2025)

- 🔄 ZK circuit verification for BTC, ETH, and Cosmos inflow/outflow  
  *(Validator-side only — no bridge clients required)*  
- 🔄 Shared vault matching interface (DOM + proof UI)  
- 🔄 ZK-audit tooling for validator-side verification  
- 🔄 Verkle Merkle root integration for exits  
- 🔄 MVP for multi-market matching board (UX + backend)  

---

## 🔜 Upcoming (Q4 2025)

- 🔜 Public Testnet Launch (BTC, USDT direct inflow via zk key model)  
- 🔜 Validator assignment via ZK DAO governance  
- 🔜 Proof bounty program (withdrawal validation + fraud testing)  
- 🔜 Full Merkle registry for inflow-eligible tokens  
- 🔜 Ecosystem partner integrations (DEXs, OTC desks, CEX interfaces)  
- 🔜 Verifier audit of ZK circuits (vault entry, withdrawal, identity)  

---

## 🛠️ Early 2026

- 🔐 Mainnet Launch (post-circuit audits + validator stress tests)  
- 📦 Token onboarding DAO live with slashing & circuit gating  
- 🌉 Parallel inflow/outflow support from Solana, L2s, and more  
- 🧠 Vault delegation via Poseidon(delegate_pubkey || vault_id)  
- 📊 Liquidity coordination layer (exposed via API for bots & CEXs)  

---

## 📈 Long-Term (2026–2027)

- 🧱 SDK for building DApps using vault-based execution  
- 🛰️ ZK module for real-time liquidity sync across chains  
- 🛡️ Formal verification of core circuits (Plonky2 + Groth16)  
- 🔁 Continuous Merkle snapshot publishing for global state sync  
- 🌍 Cross-chain vault arbitration with fraud-proof fallback  
- 🏛️ Fully decentralized Gatekeeper DAO and Validator DAO
