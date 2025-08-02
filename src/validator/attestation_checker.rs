// Domex :: validator/attestation_checker.rs
// Validates received attestation from any of the 301 quorum validators using ZK verification only (Plonky2 + Poseidon)

use crate::types::zk_attestation::ZkAttestationPackage;
use crate::utils::plonky2_verifier::verify_recursive_proof;

/// Performs full quantum-secure validation of a ZK attestation package.
/// - Verifies recursive ZK proof with Plonky2
/// - Ensures zk_root is not empty
/// - Ensures vaults_touched is populated
/// - Sanity check on attestation timestamp
pub fn validate_attestation_zk(pkg: &ZkAttestationPackage) -> bool {
    // Step 1: Verify the recursive zero-knowledge proof (Plonky2)
    let is_valid_proof = verify_recursive_proof(&pkg.zk_proof, &pkg.attestation_hash);
    if !is_valid_proof {
        return false;
    }

    // Step 2: Sanity check: zk_root must not be empty
    let zk_root_ok = !pkg.zk_root.is_empty();

    // Step 3: vaults list must not be empty
    let vaults_ok = !pkg.vaults_touched.is_empty();

    // Step 4: Timestamp must be within ±12h of current time
    let now = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(t) => t.as_secs(),
        Err(_) => return false,
    };
    let time_delta = now.abs_diff(pkg.attested_at);
    let timestamp_ok = time_delta <= 43_200; // 12 hours

    zk_root_ok && vaults_ok && timestamp_ok
}
