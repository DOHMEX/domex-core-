# 🧮 Domex Delta Law : README

> **Trustless Liquidity Limits via ZK Proofs, No Tracking, No Surveillance**

---

## 🧭 Overview

The **Domex Delta Law** is a zero-knowledge-based liquidity fairness protocol that prevents any single actor (wallet, bot, CEX, DAO, etc.) from onboarding more than ±2% of the global liquidity for any trading pair within a defined time window, **without tracking addresses or storing history**.

This is achieved by binding every deposit to a cryptographic vault identity using Poseidon hashing, and enforcing limits entirely through **ZK circuit logic**, not behavior analysis.

---

## 📜 Delta Rule Definition

> **A vault identity may not onboard more than ±2% of the global liquidity of any Domex trading pair within a 24-hour window.**

This rule is enforced:
- Using ZK proofs at the moment of onboarding
- Without checking addresses, wallet history, or origin chains
- At the cryptographic level, not behavioral

---

## 🔐 Vault Identity

Each onboarding action is tied to a **stateless cryptographic identity**:
