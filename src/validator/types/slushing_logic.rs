// src/validator/types/slushing_logic.rs // Type definitions for validator slashing logic

use crate::types::ZkAttestationPackage;

#[derive(Debug, Clone)] pub struct SlashingReport { pub validator_id: String, pub reason: String, pub evidence: ZkAttestationPackage, }

