# Domex ZK Circuit Overview

Domex uses multiple zero-knowledge circuits to replace traditional transaction logic. Each circuit is modular, auditable, and verifier-compatible, ensuring trustless execution across vault operations, identity enforcement, and bridge flows.

---

## 🔍 Circuit Categories

### 1. Onboarding Circuit (ZK Mint)

Used when onboarding assets (e.g., BTC, ETH, Cosmos) into Domex.

- Proves valid deposit occurred on source chain
- Connects deposit to Poseidon-bound vault identity
- Inputs: tx data, Merkle proof, user hash
- Output: mint commitment to vault

✅ Required by `proof_payload.json` with `"proof_type": "onboarding"`

---

### 2. Withdrawal Circuit (ZK Burn)

Used when a user exits Domex and reclaims native assets.

- Proves vault ownership via Poseidon hash
- Confirms token burn and withdrawal amount
- Outputs: burn commitment, destination address
- Used by bridge clients to trigger native transfer

✅ Required by `proof_payload.json` with `"proof_type": "withdrawal"`

---

### 3. Delegation Circuit (Optional / Future)

Used to delegate vault rights (e.g., bots, DAOs, strategies).

- Inputs: vault ID, delegator hash, delegatee public key
- Output: Poseidon(vault_id || delegate_pubkey)
- Used to temporarily authorize non-owners to act

✅ Enables off-chain bots to generate proofs with bounded rights

---

### 4. Identity Binding Circuit

- Proves the Poseidon hash commitment is valid
- Verifies user entropy + salt were used in correct format
- Used during identity creation or wallet abstraction setup

---

### 5. Merkle Root Verification

- Proves proof corresponds to a specific Merkle/Verkle root snapshot
- Prevents use of outdated or mismatched proofs
- Used by validators before accepting any state change

✅ All validator-accepted ZK circuits must be Merkle-bound

---

## 🔐 Circuit Standards

| Feature              | Status  |
|----------------------|---------|
| Groth16 Compatible   | ✅       |
| PLONK Ready          | 🔜       |
| Poseidon Inside      | ✅       |
| Merkle Root Binding  | ✅       |
| Replay Prevention    | ✅       |

---

## 🧪 Circuit Deployment Plan

- Phase 1 (Live): Onboarding + Withdrawal (Groth16)
- Phase 2: Identity and Delegation
- Phase 3: PLONK unification + recursion

---

> Domex circuits replace signatures, txs, and trust assumptions — they are the core logic units of the protocol.
