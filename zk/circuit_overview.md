# ZK Circuit Overview: Domex Vault & Identity Logic

Domex uses modular zero-knowledge circuits to replace traditional transaction logic.  
Each circuit is off-chain generatable, auditable, and verifiable by Domex global validators.  
This guarantees trustless execution for vault operations, identity enforcement, and asset onboarding/withdrawal.

---

## 🔍 Circuit Categories

### 1. Onboarding Circuit (ZK Mint)

Used to bring assets into Domex from any blockchain via **ZK proof only** — no bridges, no contracts.

- Proves user controls private key that received a valid deposit
- Binds proof to a vault via:
  
  ```text
  Poseidon(sk || vault_id || zk_node_id)
