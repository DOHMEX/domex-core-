// src/validator/slushing_logic.rs // Domex slashing logic for dishonest validator detection

use crate::types::{ZkAttestationPackage, QuorumSyncResult}; use std::collections::HashMap;

/// Represents a slashing report against a validator #[derive(Debug, Clone)] pub struct SlashingReport { pub validator_id: String, pub reason: String, pub evidence: ZkAttestationPackage, }

/// Checks each validator's attestation for inconsistency with quorum pub fn detect_slashing_violations( attestations: &[ZkAttestationPackage], quorum: &QuorumSyncResult, ) -> Vec<SlashingReport> { let mut reports = Vec::new(); let expected_hash = &quorum.attestation_hash;

for att in attestations.iter() {
    if &att.attestation_hash != expected_hash {
        reports.push(SlashingReport {
            validator_id: att.validator_id.clone(),
            reason: "Mismatched attestation hash".into(),
            evidence: att.clone(),
        });
    }
}

reports

}

/// Tallies violations and returns validators eligible for punishment pub fn slashable_validators(reports: &[SlashingReport]) -> HashMap<String, usize> { let mut tally = HashMap::new();

for report in reports {
    *tally.entry(report.validator_id.clone()).or_insert(0) += 1;
}

tally

}

