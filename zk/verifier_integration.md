# Verifier Integration: ZK Proofs and Validator Finality

This document outlines how Domex validators interact with zk-proof submissions, verify Merkle roots, and finalize vault state transitions.

---

## 🔄 ZK Proof Submission Pipeline

1. **User generates zk-SNARK proof** using local prover script or client
2. Proof is submitted to a Domex validator node with vault ID and payload
3. Validator runs ZK verifier for the specific circuit (onboarding, withdrawal, etc.)
4. If valid:
   - Updates vault state locally
   - Recomputes Merkle/Verkle root
5. Root is broadcast to validator quorum for zk-finalization

---

## 🔐 ZK Verifier Modules

Each circuit type is registered with a:
- **Verifier hash** (commitment to compiled circuit)
- **Merkle-bound scope**
- **Vault-type compatibility**

| Circuit        | Verifier Entry |
|----------------|----------------|
| Onboarding     | `verifier.onboard_groth16.sol` |
| Withdrawal     | `verifier.exit_groth16.sol`    |
| Identity check | `verifier.poseidon_auth.sol`   |

> All verifier hashes must match the Merkle onboarding registry.

---

## 🧱 Merkle/Verkle Finality Enforcement

Validators compute a new Merkle/Verkle root after each valid proof:
- Encodes vault balances
- Tracks ownership updates
- Logs burns, mints, or exits
- Ties to block height or epoch hash

Validators who sign a root with an invalid proof inside it are **automatically slashed**.

---

## ✅ zk-Finality Criteria

A ZK proof can only be finalized if:
- It is non-replayable
- It passes Poseidon identity match
- The Merkle root it’s tied to matches the current vault state
- The verifier hash is whitelisted in the registry

---

## 🧠 Integration in Rust & Solidity

- Rust: Native proof verifier wrappers for SNARK validation
- Solidity: On-chain bridge watchers may verify subset of final roots
- All validator operations are off-chain but globally published for transparency

---

## ⚠️ Slashing Risks

| Violation Type              | Result             |
|-----------------------------|--------------------|
| Invalid proof finalized     | Stake slashed      |
| Verifier mismatch           | Quorum rejection   |
| Merkle root tampering       | Blacklisting       |
| Registry bypass             | Permanent ban      |

---

> ZK proofs are not just inputs — they are state triggers. Validator honesty is enforced by cryptographic correctness.
