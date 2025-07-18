# Domex Vault Lifecycle: Deposits & Withdrawals

Domex replaces traditional transaction-based blockchains with a **ZK-proof-based vault system** for asset custody, trade coordination, and native chain settlement.

This document outlines the full asset lifecycle, from deposits (onboarding) to withdrawals (exit), using zero-knowledge proofs, not transactions or wallets.

---

## 📥 Deposits (Onboarding Phase)

Deposits are how users bring real assets (e.g., BTC, ETH) into Domex. Domex uses zk-verified bridge clients to ensure every deposit is cryptographically valid, without trusting any custodians or relayers.

### 🔁 Deposit Flow

1. **User sends native asset** (e.g., BTC) to a vault-linked bridge custody address.

2. **Bridge Client (Cosmos, ETH, etc.)**:
   - Detects the deposit
   - Generates a zk-SNARK proving:
     - Origin chain transaction
     - Amount
     - Recipient Poseidon identity
     - Target vault

3. **Proof Submission**
   ```json
   {
     "origin_chain": "Bitcoin",
     "token": "BTC",
     "amount": 1.5,
     "recipient_vault_id": "0xabc",
     "proof": "<zk-snark-proof>"
   }
