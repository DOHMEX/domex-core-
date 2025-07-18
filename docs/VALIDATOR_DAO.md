# Domex Validator DAO & zk-Finality (Step 9)

Validators in Domex are not transaction processors. They are zk-verifiers and Merkle root finalizers responsible for enforcing integrity in the vault-state execution layer.

This document outlines their roles, reward mechanics, and slashing conditions.

---

## 🧱 Role of Validators

Validators are responsible for:

- Verifying zk-SNARKs for vault onboarding and withdrawals
- Finalizing Merkle/Verkle root snapshots of vault state transitions
- Checking proof-correctness of order books and ownership updates
- Triggering bridge exits after proof validation

Validators never execute transactions, they **approve only cryptographically valid state changes.**

---

## 🧠 Validator DAO Responsibilities

- Maintain and vote on global ZK verifier versions
- Enforce registry inclusion/exclusion for tokens
- Monitor zk-circuit compliance and proof validity
- Slash misbehaving peers who finalize bad roots or fake proofs

---

## ⚙️ zk-Finality Pipeline

1. Local node submits ZK proof + vault state delta  
2. Validators verify proof matches Poseidon-bound identity + state snapshot  
3. Merkle root is updated and globally finalized  
4. Finalized state becomes the source of truth for bridge clients + trades

---

## 💰 Reward Mechanism

- Validators earn rewards per **valid ZK proof finalized**  
- Rewards are streamed via **automated, epoch-based payout contracts**  
- No MEV, no gas — purely based on **ZK work contribution**

---

## 🔨 Slashing Conditions

| Condition                        | Outcome       |
|----------------------------------|----------------|
| Finalizing invalid zk-proof      | Slashed stake |
| Skipping validator quorum step   | Timeout penalty |
| Approving a duplicate Merkle root | Slashed + banned |
| Collusion with malicious bridge  | Blacklist + DAO escalation |

Slashing is automatic and enforced by zk-proof verifiers. No social layer is needed for punishment.

---

## 🧪 Validator Staking Model

- Validators must lock stake to participate
- Slashing reduces both stake and zk-work reward share
- Delegated staking supported (via zk delegation hash)

Example hash:
```text
Poseidon(vault_id || validator_pubkey) = delegated_validator_hash
