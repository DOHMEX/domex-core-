// src/validator/proof_receiver.rs

use std::collections::HashMap;
use crate::types::proofs::ProofSubmission;
use crate::validator::attestation::ProofAttestation;
use crate::validator::slashing_engine::SlashingEngine;
use crate::validator::global_attestor::GlobalAttestor;
use crate::validator::cometbft_broadcast::CometBroadcaster;
use crate::validator::proof_registry::ProofRegistry;
use crate::common::zk_utils::verify_full_zk_proof;

/// Handles receiving and routing submitted ZK proofs from origin validators
pub struct ProofReceiver {
    pub attestor: GlobalAttestor,
    pub slashing: SlashingEngine,
    pub broadcaster: CometBroadcaster,
    pub registry: ProofRegistry,
}

impl ProofReceiver {
    /// Create a new receiver
    pub fn new(
        attestor: GlobalAttestor,
        slashing: SlashingEngine,
        broadcaster: CometBroadcaster,
        registry: ProofRegistry,
    ) -> Self {
        Self {
            attestor,
            slashing,
            broadcaster,
            registry,
        }
    }

    /// Process a proof submission from a global validator
    pub fn process_submission(&mut self, submission: ProofSubmission) -> Result<(), String> {
        // Verify the ZK proof
        if !verify_full_zk_proof(&submission.zk_proof) {
            self.slashing.mark_invalid(&submission.validator_id, "Invalid ZK proof");
            return Err("Invalid proof".into());
        }

        // Check if this validator already submitted for this vault
        if self.registry.has_submitted(&submission.validator_id, &submission.vault_id) {
            return Err("Duplicate submission".into());
        }

        // Construct attestation
        let attestation = ProofAttestation {
            vault_id: submission.vault_id.clone(),
            token: submission.token.clone(),
            size: submission.size,
            owner_hash: submission.owner_hash.clone(),
            zk_root: submission.zk_root.clone(),
            attestation_hash: submission.attestation_hash.clone(),
            timestamp: submission.timestamp,
        };

        // Record and propagate attestation
        self.registry.record_submission(&submission.validator_id, &submission.vault_id);
        self.attestor.collect(attestation.clone());

        // If quorum reached, broadcast proof attestation
        if self.attestor.has_quorum(&submission.vault_id) {
            let proof = self.attestor.build_final_attestation(&submission.vault_id);
            self.broadcaster.propagate_attestation(&proof);
        }

        Ok(())
    }
}
