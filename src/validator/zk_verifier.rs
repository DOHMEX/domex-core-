// Domex ZK Verifier – Final (No Placeholder Version)
// Connects to real Plonky2 verifier and outputs zk_root after validation

use crate::types::NormalizedProof;
use crate::utils::poseidon_hash;
use std::process::{Command, Stdio};
use std::io::Write;
use std::str;

/// Verifies a Plonky2 ZK proof and returns the resulting zk_root
/// Requires proof payload, identity, vault context, and timestamp
pub fn verify_zk_proof(proof: &NormalizedProof) -> Result<String, String> {
    // Serialize input for verifier (JSON format expected by verifier binary or RPC)
    let input_json = serde_json::json!({
        "vault_id": proof.vault_id,
        "token": proof.token,
        "size": proof.size,
        "owner_hash": proof.owner_hash,
        "timestamp": proof.timestamp,
        "zk_payload": base64::encode(&proof.zk_payload),
    });

    // Spawn external verifier process (example: ./zk_verifier binary)
    let mut child = Command::new("./zk_verifier")  // You must compile this binary ahead
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to launch ZK verifier: {}", e))?;

    {
        let stdin = child.stdin.as_mut().ok_or("Failed to open stdin")?;
        stdin
            .write_all(input_json.to_string().as_bytes())
            .map_err(|e| format!("Failed to write to verifier stdin: {}", e))?;
    }

    // Read verifier output
    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to read verifier output: {}", e))?;

    if !output.status.success() {
        let stderr = str::from_utf8(&output.stderr).unwrap_or("Invalid stderr");
        return Err(format!("ZK verification failed: {}", stderr));
    }

    // Parse zk_root (expected to be raw string)
    let stdout = str::from_utf8(&output.stdout).unwrap_or("").trim().to_string();
    if stdout.is_empty() {
        return Err("ZK verifier returned empty output.".to_string());
    }

    Ok(stdout)  // This is the zk_root
}
