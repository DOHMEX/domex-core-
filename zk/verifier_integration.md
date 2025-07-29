# Verifier Integration: ZK Proofs and Validator Finality

This document outlines how Domex validators interact with zk-proof submissions, verify Merkle/Verkle roots, and finalize vault state transitions — without relying on bridge contracts or external custody logic.

---

## 🔄 ZK Proof Submission Pipeline

1. User generates a zk-SNARK proof using their local zk client or prover.
2. The proof is submitted to a Domex validator node along with:
   - `vault_id`
   - `proof_type` (e.g., onboarding, withdrawal)
   - Poseidon identity
   - ZK proof components (`a`, `b`, `c`)
   - Merkle root snapshot
3. The validator:
   - Verifies the ZK proof against the corresponding circuit verifier
   - Checks the Merkle root matches its current local vault state
   - Confirms Poseidon identity matches expected vault owner
4. If all checks pass:
   - The validator updates vault state (mint, burn, transfer)
   - Recomputes a new Merkle/Verkle root
   - Broadcasts this root to the validator quorum for zk-finalization

---

## 🔐 ZK Verifier Modules

Each registered circuit (e.g., onboarding, withdrawal) is associated with:

| Component              | Purpose                                               |
|------------------------|-------------------------------------------------------|
| `verifier_hash`        | Hash commitment of circuit verifier (Groth16, PLONK)  |
| `vault_type`           | Indicates which vaults the circuit applies to         |
| `merkle_scope`         | Ensures proofs bind to specific Merkle root snapshot  |

Example verifier entries:

| Circuit        | Verifier Hash ID            |
|----------------|-----------------------------|
| Onboarding     | `onboard_verifier_hash`     |
| Withdrawal     | `withdrawal_verifier_hash`  |
| Identity Check | `poseidon_auth_verifier`    |

All verifier hashes must be registered and whitelisted via Merkle onboarding registry to prevent circuit forgery.

---

## 🧱 Merkle/Verkle Finality Enforcement

After accepting a valid proof, the validator updates its internal Merkle or Verkle root:

- Encodes new vault balances and token burns/mints
- Tracks changes in vault ownership (Poseidon identities)
- Commits new root to validator memory, tied to timestamp or epoch
- Broadcasts updated root for quorum agreement

Any validator that signs or proposes a root containing:
- An invalid proof,
- A forged verifier hash,
- Or a Merkle mismatch,

...is **automatically slashed** by the protocol.

---

## ✅ zk-Finality Criteria

A zk-proof is finalized only if:

- ✅ It is cryptographically valid (Groth16 or PLONK)
- ✅ It is bound to the current Merkle/Verkle root
- ✅ It passes replay prevention (nonce/timestamp check)
- ✅ The Poseidon identity proves ownership of the vault
- ✅ The verifier hash is approved in the registry

---

## 🧠 Integration Logic: Rust & Optional Solidity

| Language | Role                                                                 |
|----------|----------------------------------------------------------------------|
| Rust     | Domex validators use native Arkworks or Noir-based verifiers         |
| Solidity (optional) | External DAOs or bridge watchers can verify finalized Merkle roots off-chain |

Domex validator operations are fully **off-chain** but publicly visible.  
Validators publish:
- Finalized Merkle roots
- Circuit hashes
- Proof payload metadata (anonymized)

This allows external verifiers (e.g., CEXs, DAOs) to independently audit finality.

---

## ⚠️ Slashing & Validator Integrity

| Violation Type            | Consequence             |
|---------------------------|--------------------------|
| Invalid zk-proof finalized| Immediate slashing       |
| Verifier hash mismatch    | Quorum rejection         |
| Root tampering            | Validator blacklisting   |
| Registry bypass attempt   | Permanent validator ban  |

---

## 🛑 Domex Finality Disclaimer

Domex does **not** verify zk clients, wallets, or key management tools.  
It only finalizes proofs that:

- Match a vault schema
- Reference a valid Poseidon-bound identity
- Align with the validator’s current Merkle state

Bridge execution, wallet custody, and withdrawal scripts are **external** to Domex and not controlled by the protocol.

---
