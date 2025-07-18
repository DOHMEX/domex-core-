# Vault Settlement and Trade Rules.

Domex vaults act as unified trading boards. They are not user wallets but pooled market boards governed by Raft-style consensus across five local nodes.

## Phase 1: Token Representation.
- When a token (e.g. BTC) is sent to a Domex-mapped address, a ZK client confirms it.
- Global validators verify and mint dToken (e.g. dBTC), recorded in Merkle state.
- The token is **not yet in a vault**. It is in the global state as owned by the sender.

## Phase 2: Vault Binding for Trading.
- The user or CEX chooses a vault (e.g. BTC/USDT) by submitting a trade intent with a ZK proof.
- The token is now visible in that vault’s order board.
- Vaults do not move tokens. They reflect matching rights and settle balances accordingly.

## Matching Engine.
- Local Raft nodes maintain real-time trade matching inside each vault.
- Final settlement is confirmed by Merkle root updates validated by global validators.
