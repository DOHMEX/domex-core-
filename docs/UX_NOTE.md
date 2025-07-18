# UX Note: DOM, Charts & Proof-Based Interaction

Domex is fundamentally different from traditional blockchains in that it does not use transactions, wallets, or mempools. Instead, all market interaction happens through **off-chain interfaces** that trigger **zk-proof-based state changes**.

This document outlines the intended user experience design for interacting with Domex markets.

---

## 🎯 UX Goals

- Provide real-time **Depth of Market (DOM)** visuals
- Enable proof-based actions (e.g., trade, withdraw) without needing a wallet
- Support institutional-grade market tools (charts, order books, APIs)
- Abstract away zk complexity from the user

---

## 📊 DOM and Order Book Experience

- All DOM views are rendered from vault-based order matching activity
- Matching happens locally per vault (via Raft) and is synced globally by zk-merkle root
- DOM is reconstructed client-side from verified orders + local consensus snapshots
- DOM can be streamed via an open API for DEXs, bots, or traders

Users will experience a **fully interactive trading UI** (DOM, price ladder, chart, etc.), but all backend actions will occur via ZK proof submissions, not transactions.

---

## 🧩 Walletless UX

- Users don't connect Metamask or sign transactions
- Instead, their identity is derived from a **Poseidon commitment hash**
- The client generates zk-proofs of intent using a lightweight script and signs with identity hash
- No private key broadcasting, no transaction fee approval

---

## 🧪 Example Flow

1. **User places order:**  
   Fills out JSON order form → Client generates zk-proof → Submits to local node

2. **Matching occurs:**  
   Local Raft consensus matches buyer/seller → Vault state updates

3. **User sees trade settled:**  
   Frontend DOM + charts update in <500ms based on updated vault snapshot

4. **User withdraws:**  
   Frontend provides "Withdraw" button → Client burns dToken + submits zk proof

---

## 🧠 Developer Tools (Planned)

- JS/TS SDK for proof-based DOM interfacing
- WASM zk-client for browser proof generation
- DOM + chart libraries for CEX/DEX integration
