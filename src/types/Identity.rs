// ===============================
// types/Identity.rs Types (Poseidon / Delegation)
// ===============================

/// Unique vault identifier (usually UUID or vault Merkle root)
pub type VaultId = String;

/// Poseidon hash representing vault ownership identity (hex-encoded)
pub type PoseidonHash = String;

/// Poseidon hash representing delegated authority for trading or exit rights
pub type DelegationHash = String;
