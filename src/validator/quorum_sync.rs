// Domex Validator Quorum Sync
// Aggregates 301 validator attestations and ensures hash-level consensus

use crate::types::{ZkAttestationPackage, QuorumSyncResult};
use std::collections::HashMap;

/// Syncs validator attestations and checks for exact hash match across the 301 quorum
pub fn sync_attestation_quorum(
    incoming_attestations: Vec<ZkAttestationPackage>,
) -> Result<QuorumSyncResult, String> {
    if incoming_attestations.len() != 301 {
        return Err("Invalid quorum size: expected 301 attestations".into());
    }

    // Count how many validators submitted each attestation hash
    let mut hash_counts: HashMap<String, u32> = HashMap::new();
    for att in &incoming_attestations {
        *hash_counts.entry(att.attestation_hash.clone()).or_insert(0) += 1;
    }

    // Identify the most frequent attestation hash
    let (majority_hash, count) = hash_counts
        .iter()
        .max_by_key(|entry| entry.1)
        .ok_or("Failed to compute majority hash")?;

    // Require perfect match across 301 validators
    if *count != 301 {
        return Err("Attestation quorum failed: inconsistent hashes".into());
    }

    // Return the first attestation matching that hash as canonical
    let canonical = incoming_attestations
        .into_iter()
        .find(|a| a.attestation_hash == *majority_hash)
        .ok_or("Canonical attestation not found in quorum")?;

    Ok(QuorumSyncResult {
        zk_root: canonical.zk_root,
        attestation_hash: canonical.attestation_hash,
        timestamp: canonical.timestamp,
        vault_id: canonical.vault_id,
        token: canonical.token,
        size: canonical.size,
        owner_hash: canonical.owner_hash,
        validator_ids: vec![canonical.validator_id], // Optional: extend to collect all if needed
    })
}
