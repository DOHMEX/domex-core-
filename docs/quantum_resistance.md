# Domex Quantum-Resistant Custody Model

Domex introduces a quantum-resistant custody and settlement framework by eliminating private key-based control from its core protocol. Instead, Domex enforces all vault actions (onboarding, settlement, and withdrawal) using Zero-Knowledge Proofs (ZKPs) and Poseidon-hashed vault identities — completely bypassing traditional key-signature schemes like ECDSA.

---

## 🔐 1. What Domex Secures (and What It Doesn’t)

Domex provides **quantum safety for custody and control**, but it does **not modify the base-layer cryptography** of Bitcoin, Ethereum, or other source chains.

| Layer                        | Quantum-Safe? | How Domex Enforces It                  |
|-----------------------------|---------------|----------------------------------------|
| Vault custody               | ✅ Yes        | ZK proofs + Poseidon-bound vault keys  |
| Onboarding flow             | ✅ Yes        | Merkle-bound proof validation          |
| Native blockchain protocol  | ❌ No         | Still uses ECDSA, not changed by Domex |

---

## 🧠 2. The ZK Key Model

Domex allows users to generate receiving addresses (e.g., BTC, ETH) using a local ZK client:

```plaintext
sk = random_secret()
pk = EC_multiply(sk)   ← public key used for bridge
