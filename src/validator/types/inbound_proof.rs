// Domex Types – Inbound Proof Structures

/// Raw proof payload sent from a local node to global validator
#[derive(Debug, Clone)]
pub struct IncomingProof {
    pub vault_id: String,
    pub token: String,
    pub size: u64,
    pub owner_hash: String,     // Poseidon hash (hex string)
    pub zk_payload: Vec<u8>,    // Serialized ZK proof blob
    pub timestamp: u64,
}

/// Normalized version of the incoming proof after validation
#[derive(Debug, Clone)]
pub struct NormalizedProof {
    pub vault_id: String,
    pub token: String,
    pub size: u64,
    pub owner_hash: String,
    pub zk_payload: Vec<u8>,
    pub timestamp: u64,
}
