# Quantum-Safe Identity System (Poseidon Hash Commitment)

Domex does not use wallets, ECDSA signatures, or public-private keypairs. Instead, all user and validator identity is defined by **Poseidon hash commitments**, ensuring long-term quantum resistance and seamless ZK proof integration.

---

## 🔒 Identity = Poseidon Commitment

Each Domex identity is represented as:

```text
poseidon_hash = Poseidon(seed || context)
