# Domex Validator DAO & zk-Finality (Step 9)

Validators in Domex are not transaction processors. They are zk-verifiers and Merkle root finalizers responsible for enforcing integrity in the vault-state execution layer.

This document outlines their roles, reward mechanics, and slashing conditions.

---

## 🧱 Role of Validators

Validators are responsible for:

- Verifying zk-SNARKs for vault onboarding and withdrawals  
- Finalizing **Merkle root** snapshots of vault state transitions  
- Checking proof-correctness of order books and ownership updates  
- Triggering outbound asset release *after valid proof and Merkle finality*

Validators **never execute transactions**, they approve only cryptographically valid state changes enforced by ZK proofs.

---

## 🧠 Validator DAO Responsibilities

- Maintain and vote on approved ZK verifier hashes  
- Enforce token onboarding through circuit registry  
- Monitor zk-circuit compliance and vault integrity  
- Slash any validator that finalizes invalid proofs or mismatched roots

---

## ⚙️ zk-Finality Pipeline

1. Local node submits ZK proof + vault state delta  
2. Validator checks Poseidon-bound identity match and Merkle diff  
3. A new Merkle root is computed and signed  
4. Finalized Merkle root becomes the official state for all vaults  
5. Any bridge or client may trust this root without calling smart contracts

There are **no outbound transactions** within Domex  finality is enforced **entirely through Merkle-bound zk-proofs.**

---

## 💰 Reward Mechanism

- Validators earn fixed rewards per **valid zk-proof finalized**  
- Rewards are distributed via automated epoch-based streaming  
- No MEV, no gas — strictly tied to ZK verification work

---

## 🔨 Slashing Conditions

| Violation Type                    | Result                  |
|----------------------------------|--------------------------|
| Invalid ZK proof accepted        | Stake slashed            |
| Skipping quorum validation       | Penalty applied          |
| Finalizing mismatched Merkle root| Slashed + blacklisted    |
| Verifier registry bypass         | Permanent ban            |

Slashing is **automatic**, enforced at the zk-verifier and Merkle finality level. No social coordination required.

---

## 🧪 Validator Staking Model

- Validators must bond stake to join finality quorum  
- Slashing reduces stake and validator reputation  
- Delegated staking supported via Poseidon-based binding

```text
Poseidon(vault_id || validator_pubkey) = delegated_validator_hash
