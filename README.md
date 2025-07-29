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
  Market actions (trade, withdraw, onboard) are triggered by zero-knowledge proofs tied to vault state transitions.  
  → No transactions. No wallets. No gas.

- **Global Liquidity Layer**  
  Vaults function as pooled order boards connected via local matching engines and globally finalized by ZK-verified Merkle roots.

- **Quantum-Safe Identity**  
  Domex replaces private keys with Poseidon-based zk identities. No ECDSA. No signing. Fully quantum-resistant.

- **Validator zk-Finality**  
  Global validators finalize state only when zk-SNARK proofs are valid and Merkle root transitions are cryptographically correct.  
  → Validators do not run transactions — they verify, finalize, and publish state.

- **Local Matching Engine**  
  Vaults are operated by Raft-style local consensus for microsecond-level trade execution, with finality handled globally.

---

## ✅ What Domex *Does Not* Use

- ❌ No smart contracts  
- ❌ No wallets or keys  
- ❌ No mempools  
- ❌ No transaction fees  
- ❌ No on-chain bridges or wrapped assets  

All market interaction occurs through off-chain zk clients that submit proofs of ownership, trade intent, or withdrawal — validated by the protocol.

---

## 🧩 Proof-Driven Cross-Chain Access

Assets like BTC, ETH, and others can enter Domex *without bridges*. Instead:

- Client generates zk-proof of a valid on-chain deposit (e.g., BTC tx)
- Submits to Domex validator
- Validator verifies and mints dToken to the user’s vault
- Exit requires vault burn + zk proof of identity and withdrawal

Domex never holds custody — validators only verify. Bridges are external scripts, not Domex components.

---

## 🔗 Resources

- 📜 **Whitepaper**: [View PDF](https://drive.google.com/file/d/1sRQ8050Pi_HmoTVlU0yaIoionKzciMew/view?usp=drive_link)  
- 🎥 **Demo Video**: [YouTube](https://www.youtube.com/watch?v=h6hQoVIQCpM)  
- 🧠 **DoraHacks BUIDL**: [Domex on Dora](https://dorahacks.io/buidl/28435)  
- 🧠 **ETHGlobal Showcase**: [Domex Protocol](https://ethglobal.com/showcase/domex-protocol-qh6zh)

---

## 🛣️ Roadmap (2025)

- ✅ Vault-state execution model finalized  
- 🔄 Quantum-safe zk identity and Poseidon commitment implemented  
- 🔄 Trade intent schema + off-chain proof structure complete  
- 🔄 Validator zk-Merkle finality integration  
- 🔄 Verifier registry and validator DAO ruleset  
- 🔄 MVP DOM + matching engine UI  
- 🔜 Public testnet launch (BTC, USDT)  
- 🔜 zk-audit tooling and external verifier dashboards  
- 🔜 Proof-based token onboarding governance  
- 🔜 Global validator onboarding via zkDAO

---

## 🧑‍💻 Join the Team

We welcome contributors in:

- zk-SNARK circuit development (Groth16, PLONK)  
- Rust/CosmWasm integration for validator tooling  
- zk-bridge clients and state verifiers  
- Trading UI and DOM integration (TypeScript, WASM)

> Domex is open-source, zk-native, and permissionless by design. Built for proof-first liquidity.

---

⚠️ **Disclaimer**  
Domex is under active development. Code in this repository is experimental, unaudited, and not production-ready.
