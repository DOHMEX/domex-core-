# Vault-Based DApp Model for Domex

Domex does not use smart contracts. Instead, it supports DApps through modular vaults, where logic is enforced by zero-knowledge proofs (ZKPs) verified by local nodes.

## Overview

Every vault in Domex is treated as a logic module. Each module is tied to:
- A vault ID (e.g., BTC/USDT)
- A ZK circuit hash (defines valid proof structure)
- Optional metadata (UI, pricing model, etc.)

DApps interact with Domex by submitting ZKPs that satisfy the vault's registered logic schema.

## Architecture

1. **Proof-Based Triggers**
   - Users submit ZKPs proving:
     - Ownership of tokens
     - Valid intent (e.g., sell BTC)
     - Correct circuit format
   - If proof passes, it is accepted by the local node's Raft engine and included in vault state.

2. **Vault Registration**
   Developers register new vaults by:
   - Submitting a vault ID
   - Linking it to a circuit hash
   - Optionally submitting UI helpers (e.g., OTC dashboard)

3. **Execution Flow**
   - Proof submitted → local node validates → vault state updates
   - If Raft consensus passes, update is sent to global validators
   - Global validator confirms via ZK Merkle inclusion and finalizes the update

## Benefits

- **Composable Market Apps:** Anyone can launch DApps using existing vault schemas
- **No Deployment:** No need to deploy or compile contracts
- **ZK-Verified Logic:** All DApp logic is proof-enforced, not code-executed
- **Scalable:** Vaults operate independently, handled by local Raft nodes

## Examples

- `vault_id: "0x01BTCUSDT"`  
  → Linked to OTC circuit schema, allows token-for-token trading with pricing logic

- `vault_id: "0x02PREDICT"`  
  → Linked to binary outcome circuit, used for decentralized predictions

- `vault_id: "0x03DAO"`  
  → Linked to vote-based circuit, for DAO-style commitment execution

## Summary

Domex replaces smart contracts with vault-bound proof logic. This creates a scalable, modular framework for permissionless DApps, fully verified with ZKPs and finalized through the Domex consensus layers.
