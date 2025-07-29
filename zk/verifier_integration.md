# Verifier Integration: ZK Proofs and Validator Finality

This document defines how Domex validators interact with zero-knowledge proofs, verify vault state transitions, and finalize Merkle roots. Unlike traditional chains, Domex does not execute transactions or rely on bridge custody contracts. All logic is driven by cryptographic proof.

---

## 🔄 Proof Submission & Validation Flow

All actions in Domex (onboarding, withdrawal, ownership change) occur via a submitted ZK proof. The validator flow is:

1. **User generates a ZK proof** locally (Plonky2)
2. **User submits proof payload**, including:
   - `vault_id`
   - `proof_type` (e.g., `onboard`, `withdraw`)
   - `poseidon_identity`
   - `proof_components` (Plonky2 fields)
   - `merkle_root_snapshot`
3. **Validator verifies:**
   - Proof validity (Plonky2 circuit)
   - Identity match (Poseidon binding)
   - Merkle root match to current local vault state
4. If valid:
   - Vault state is updated
   - Merkle root is recomputed
   - New root is broadcast to validator quorum

---

## 🔐 Registered ZK Verifiers (Plonky2 Only)

Each circuit type is registered with a:

| Component        | Description                                   |
|------------------|-----------------------------------------------|
| `verifier_hash`  | Hash of compiled Plonky2 circuit verifier     |
| `vault_scope`    | Which vaults the circuit is allowed to mutate |
| `merkle_scope`   | Ensures root binding for replay protection    |

**Circuit Types:**

| Circuit       | Verifier Hash (Example)     |
|---------------|-----------------------------|
| Onboarding    | `plonky2_onboard_hash`      |
| Withdrawal    | `plonky2_withdraw_hash`     |
| Identity Auth | `plonky2_poseidon_auth_hash`|

All circuits must be pre-approved in the Domex Merkle-based verifier registry.

---

## 🧱 Merkle Root Finalization

After accepting a proof, validators:

- Encode updated vault state (balances, burns, ownership)
- Recompute Merkle root from vault tree
- Broadcast the new root to other validators
- Participate in quorum-based ZK finalization

If 67% of validators attest the same root with valid proof lineage, the root becomes globally final.

---

## ✅ zk-Finality Criteria

A proof is considered final only if:

- ✅ Valid Plonky2 proof matches registered circuit
- ✅ Bound to current Merkle root snapshot
- ✅ Includes replay protection via nonce/timestamp
- ✅ Identity matches vault ownership (Poseidon-hashed)
- ✅ Root is approved by validator quorum

---

## 🛠️ Validator Integration (Rust Only)

Domex validators run **native Rust** verification logic using Plonky2. There is no Solidity fallback.

| Layer      | Role                             |
|------------|----------------------------------|
| Rust (Ark/Plonky2) | All zk-verification (in-memory) |
| DOM Clients | Consume finalized root snapshots |

All validator operations are **off-chain** but globally visible. Roots and proofs are published for public auditing.

---

## 🔨 Slashing Conditions

| Violation                      | Penalty             |
|-------------------------------|---------------------|
| Finalizing invalid proof       | Immediate slash     |
| Verifier hash not whitelisted | Quorum rejection    |
| Invalid Merkle root proposed  | Blacklist & ban     |
| Identity mismatch             | No finality granted |

---

## 🛑 Protocol Scope

Domex **does not** verify zk wallets, bridges, or frontends. It only finalizes:

- Valid Plonky2 proofs
- Poseidon-bound vault ownership
- Registered verifier circuits
- Correct Merkle root updates

Any external tools must comply with Domex proof format and schema.
