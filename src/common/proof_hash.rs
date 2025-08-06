// ==========================================================
// proof_hash.rs — Domex ZK Proof Hashing and Attestation Root
// ==========================================================
//
// Computes the unique Poseidon-based hash for a validated proof,
// which is used as the input to the final validator attestation.
// All validators must produce identical proof hashes for convergence.
//

use crate::poseidon_utils::poseidon_hash;
use crate::common_types::{MerkleRoot, VaultId, DelegationHash};
use crate::token_config::MIN_PROOF_FUEL_BURN;

/// The unified data needed to construct a proof attestation hash
pub struct ProofAttestationInput {
    pub zk_proof_hash: [u64; 3],
    pub vault_state_root: MerkleRoot,
    pub fuel_burn_root: [u64; 3],
    pub delegation_hash: DelegationHash,
    pub withdrawal_target: [u64; 3],
}

impl ProofAttestationInput {
    /// Compute the final Poseidon-based attestation root
    pub fn compute_attestation_root(&self) -> [u64; 3] {
        poseidon_hash(&[
            self.zk_proof_hash[0],
            self.zk_proof_hash[1],
            self.zk_proof_hash[2],
            self.vault_state_root.0,
            self.fuel_burn_root[0],
            self.fuel_burn_root[1],
            self.fuel_burn_root[2],
            self.delegation_hash.0,
            self.withdrawal_target[0],
            self.withdrawal_target[1],
            self.withdrawal_target[2],
        ])
    }
}
