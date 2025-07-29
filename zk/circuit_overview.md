# Domex ZK Circuit Overview

Domex uses modular zero-knowledge circuits to replace traditional transaction logic.  
Each circuit is auditable, proof-generatable off-chain, and verifiable by Domex global validators, ensuring trustless execution for vault operations, identity enforcement, and token onboarding/withdrawal.

---

## 🔍 Circuit Categories

### 1. Onboarding Circuit (ZK Mint)
Used to onboard assets from any blockchain into Domex using only zk-proofs — no bridge logic required.

- Proves the user controls the private key that received a valid deposit
- Ensures the address is vault-bound via:  
  `Poseidon(sk || vault_id || zk_node_id)`
- Inputs: tx metadata, Merkle receipt (if applicable), zk proof of `sk`
- Output: mint commitment to the correct vault
- This replaces chain-specific bridges. Any zk-client can generate a valid proof.
✅ Used with `proof_payload.json` containing `"proof_type": "onboarding"`

---

### 2. Withdrawal Circuit (ZK Burn)
Used when a user exits Domex and reclaims native assets on the origin chain.

- Proves vault ownership using Poseidon-bound zk identity
- Confirms token burn and withdrawal intent
- Outputs: burn commitment + withdrawal destination (e.g., BTC address)
- Validators verify this proof before authorizing external bridge execution
✅ Domex does **not** operate bridge contracts; zk validation is off-chain
✅ Used with `proof_payload.json` containing `"proof_type": "withdrawal"`

---

### 3. Delegation Circuit (Optional / Future)
Used to delegate limited vault rights to bots, DAOs, or strategies.

- Inputs: vault ID, delegator hash, delegatee public key
- Output: `Poseidon(vault_id || delegate_pubkey)`
- Enables non-owner zk-clients to act temporarily on behalf of the owner
✅ Prevents full custody loss while allowing automated proofs

---

### 4. Identity Binding Circuit
Verifies that vault-bound zk identity is valid and securely generated.

- Proves that user entropy and vault ID were hashed correctly
- Prevents identity collisions, impersonation, or reused keys
- Used during onboarding and zk client setup

---

### 5. Merkle Root Verification
Ensures all zk-proofs are tied to an up-to-date state snapshot.

- Proves the zk-proof references the correct Merkle or Verkle root
- Ensures vault mutations are consistent with the validator's root history
✅ All validator-accepted ZK circuits must be Merkle-bound

---

## 🔐 Circuit Standards

| Feature               | Status   |
|-----------------------|----------|
| Groth16 Compatible    | ✅ Yes   |
| PLONK Ready           | 🔜 Phase 2 |
| Poseidon Inside       | ✅ Yes   |
| Merkle Root Binding   | ✅ Yes   |
| Replay Prevention     | ✅ Yes   |

---

## 🧪 Circuit Deployment Plan

| Phase        | What’s Deployed                    |
|--------------|------------------------------------|
| Phase 1      | ✅ Onboarding + Withdrawal (Groth16) |
| Phase 2      | 🟡 Identity & Delegation           |
| Phase 3      | 🔜 PLONK migration + recursive unification |

---

## 🔁 Legacy Bridge Logic Removal

Domex no longer requires or manages chain-specific bridge implementations.  
All asset onboarding and exits are handled by zk-clients through zk-proofs.  
Validators enforce correctness via Poseidon identity, Merkle verification, and ZK circuits — not by reading or signing from bridge contracts.

This ensures maximum flexibility, security, and compatibility with any key-based blockchain.
