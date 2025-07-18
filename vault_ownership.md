# Vault-Bound Ownership Enforcement

Domex enforces vault-level ownership using Poseidon-based identity commitments and zero-knowledge proof validation.

## Ownership Model

- When a user bridges BTC (or any native asset), it becomes dBTC in Domex.
- The claim right is linked to the vault entry and Poseidon identity.
- Vault entries are Merkle-anchored and validated via ZK proofs.

## Withdrawal Rights

- Only the current vault-bound identity (Poseidon-based hash) can submit a ZK proof to exit.
- Once dBTC is sold, the previous owner **loses all ability to reclaim BTC**.
- ZK proof must include:
  - Correct identity hash
  - Valid vault record inclusion
  - Proof of trade or ownership transition

## Security Outcome

- Prevents unauthorized withdrawals
- Eliminates replay or double-claim attacks
- Guarantees asset ownership is tightly bound to trade state

## Implementation Notes

- Identity = `Poseidon(private_key)`
- Ownership enforced via ZK circuit checking:
  - Merkle inclusion proof (vault state)
  - Identity hash matching vault record
