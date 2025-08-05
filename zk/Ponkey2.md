# Ponkey2

> **Post-Quantum ZK Identity Stack for Domex**

Ponkey2 is a zero-knowledge proof (ZKP) stack designed for use within the Domex blockchain. It enables post-quantum secure onboarding, vault authorization, and identity verification using a hybrid of **Polaris**-style transparent zkSNARKs and **Poseidon** hash functions.

Ponkey2 is named from its core ingredients:
- **Po**seidon
- **N**ative zk identity logic
- **Key**-linked proofs
- Inspired by **Polaris**’s IOP + FRI architecture

---

## 🔐 Key Goals

- ✅ **Post-quantum security** (no elliptic curve assumptions)
- ✅ **No trusted setup** (transparent proof system)
- ✅ **Vault-binding identity** via Poseidon(sk || vault_id || zk_node_id)
- ✅ **On-chain verifiability** with Merkle and identity hash checks
- ✅ **Customizable for trading, delegation, BTC onboarding**

---

## 🧱 Core Architecture

| Component          | Description                                                                 |
|--------------------|-----------------------------------------------------------------------------|
| `Poseidon Hash`    | Cryptographic hash function optimized for ZK circuits and Pasta fields      |
| `Polaris zkSNARK`  | Transparent, hash-based proof system with polylog verification              |
| `Identity Circuit` | ZK circuit proving knowledge of secret key bound to vault and zk-node ID    |
| `Public Inputs`    | Includes Poseidon identity hash, vault ID, zk-node ID, pubkey (x, y), etc.  |
| `Private Inputs`   | 32-byte secret key (`sk_bytes`)                                             |

---

## 🧪 Example Flow

1. **User Generates Identity**
   - Input: secret key `sk`, `vault_id`, `zk_node_id`
   - Computes: `Poseidon(sk || vault_id || zk_node_id)`

2. **User Generates Proof**
   - Proves knowledge of `sk` that produces correct Poseidon hash
   - Uses Polaris-style prover over hash-based commitments

3. **User Submits ZK Proof**
   - Includes public inputs:
     - `identity_hash`
     - `vault_id`
     - `zk_node_id`
     - Public key `(x, y)`
     - Optional: deposit chain & tx hash
   - Validators verify proof using lightweight verifier
   - If valid, identity is authorized for trading or withdrawal

---

## 📦 File Structure

```bash
ponkey2/
├── client_identity.rs      # Computes Poseidon(sk || vault_id || zk_node_id)
├── deposit_address.rs      # Derives public key (x, y) from sk (if needed)
├── hash_utils.rs           # Poseidon wrapper and field conversion utilities
├── proof_generator.rs      # Polaris-compatible proof generator (client side)
├── circuit_interface.rs    # Stub for connecting to Polaris circuit executor
├── types/
│   └── zk_client.rs        # Structs for public/private inputs & onboarding request
├── README.md               # This file
