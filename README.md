# Domex

**ZK-native blockchain for global liquidity coordination**

Domex replaces traditional transaction-based blockchains with a vault-state execution model triggered by zero-knowledge proofs. This unlocks highly scalable, verifiable, and permissionless market coordination.

---

## 🔍 Project Vision

Markets today are fragmented. Liquidity is siloed across blockchains, apps, and exchanges, with no unified access or composability.

**Domex introduces a universal, shared order book secured by zk-proofs.** It creates a permissionless financial layer where market activity is triggered by cryptographic proofs — not transactions or smart contracts.

---

## 🧠 Core Innovations

- **Vault-State Execution**  
  Market actions are triggered by zero-knowledge proofs tied to vault state transitions. No transactions. No gas.

- **Global Liquidity Layer**  
  Unified, permissionless order book for DeFi and OTC flows — across chains.

- **Quantum-Safe Identity**  
  Uses Poseidon hash-based identities (no ECDSA). Domex is quantum-resistant by default.

- **Validator zk-Finality**  
  Finality and slashing enforced through zk-SNARK circuits and Merkle state verification.

- **Local Matching Engine**  
  DApps operate real-time Raft-style consensus for vault logic, with global zkMerkle finality.

---

## 🌐 Cross-Chain Support

Domex supports proof-based asset bridges via:

- Cosmos (client-side zk)
- Ethereum
- Solana (planned)
- More via custom zk clients

Bridged assets are verified with client-side ZK proofs and made tradable in the global vault layer.

---

## 🔗 Resources

- 📜 **Whitepaper**: [View PDF](https://drive.google.com/file/d/1RMk1m5Gdf2j2qI8C4HQVNZuIdbX-yCbT/view?usp=drive_link)
- 🎥 **Demo Video (70s)**: [YouTube](https://www.youtube.com/watch?v=h6hQoVIQCpM)
- 📘 **DoraHacks BUIDL**: [Domex on Dora](https://dorahacks.io/buidl/28435)

---


## 🛣️ Roadmap (2025)

- ✅ Protocol design finalized  
- ✅ ZK identity + vault execution implemented  
- 🔄 Client-side proof bridge circuits (Cosmos, Ethereum)  
- 🔄 MVP shared vault matching interface  
- 🔜 zk audit tools and verifier integration  
- 🔜 Community validator onboarding  

---

## 🧑‍💻 Join the Team

We welcome contributors in:

- zk-SNARK circuit development  
- Rust/ZK scripting  
- Cosmos proof bridge logic  
- UX for proof-based trading  

> This repo is maintained by the Domex core team. All components are experimental and unaudited.

---

⚠️ **Disclaimer**  
Domex is under active development. Code and logic in this repo are not production-ready and are unaudited.
