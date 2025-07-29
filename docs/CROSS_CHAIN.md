# Domex Cross-Chain ZK Proof Model (Step 10)

Domex does **not** operate bridges or custody contracts.  
Instead, it enforces asset custody through **zero-knowledge proof validation** submitted by external zk-clients.

This design keeps Domex trustless, auditable, and quantum-resistant — while allowing any bridge, wallet, or system to plug into Domex vaults **as long as it complies with ZK proof standards.**

---

## ✅ Key Principles

- **No Bridge Contracts**: Domex never locks or holds tokens on any L1 or L2.
- **ZK Proofs Only**: Domex validators accept state changes *only* from valid zero-knowledge proofs.
- **External zk-Clients**: Any zk-compatible client (e.g., Cosmos, Ethereum) may submit asset proofs.
- **Vault-Centric Logic**: Vault balances are updated via proof verification — not transactions.

---

## 🔄 Asset Inflow (ZK Mint)

1. User sends BTC/ETH/etc. to a zk-generated address
2. A zk-client (off-chain or light client) generates a ZK proof of deposit
3. User submits the proof to Domex
4. Validators verify proof, update Merkle state, and mint dToken to the correct vault

✔️ Domex validators never touch the asset  
✔️ Vault identity is bound to Poseidon identity hash in the proof

---

## 🔁 Asset Outflow (ZK Burn)

1. Vault owner generates proof of vault ownership and burn intent
2. Proof includes Merkle root, Poseidon ID, and withdrawal target
3. Validators verify the proof and finalize the Merkle root
4. External bridge client (not Domex) observes root and triggers release on-chain

✔️ Exit is trustless — Domex enforces proof rules  
✔️ Domex does not release funds; it verifies vault burns only

---

## 🔐 Security Model Summary

| Component                | Role of Domex                        |
|--------------------------|--------------------------------------|
| Native Asset Deposit     | ❌ Not handled by Domex              |
| Proof Generation         | ❌ Done by user/bridge/wallet        |
| Proof Submission         | ✅ Submitted to Domex validators     |
| Proof Verification       | ✅ Full zk-verifier module in Domex  |
| Vault Update             | ✅ Done on valid proof only          |
| Native Asset Release     | ❌ Handled by external clients       |

---

## 🧠 Why This Matters

Domex does **not try to replace bridges** — it replaces **trust** in bridges.  
As long as the proof is valid, Domex doesn’t care who builds the wallet, relay, or submission client.

This allows:
- Open interoperability with any zk-bridge
- Plug-and-play for wallets, OTC desks, and CEXs
- Zero lock-in or protocol risk from buggy contracts

---

## 📎 External zk-Client Files (Optional Templates)

If developers wish to write zk-clients that feed into Domex, they may still reference:
- `zk/sample_proof.json`
- `zk/circuit_overview.md`
- `core/identity_poseidon.md`

These provide examples of the valid proof structure required by Domex.

---

> Domex is a ZK-based validator layer — not a bridge, not a wallet, not a signer.
> It defines what is **provably valid**, not how proofs are built.
