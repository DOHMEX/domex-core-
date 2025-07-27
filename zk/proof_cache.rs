// ===============================
// zk/proof_cache.rs : Domex ZK Proof Fallback Storage
// ===============================

use std::fs::{OpenOptions, File};
use std::io::{Write, BufReader};
use std::path::Path;
use std::sync::Mutex;
use crate::types::zk::ZkProofInput;
use serde_json;

/// Local path for cached proof inputs (JSON lines)
const CACHE_FILE: &str = "/tmp/domex_proof_cache.jsonl";

lazy_static::lazy_static! {
    static ref CACHE_LOCK: Mutex<()> = Mutex::new(());
}

/// Stores a ZK proof input to local disk.
/// Used by non-leader nodes to ensure replayability or failover.
pub fn store_proof_input(input: &ZkProofInput) {
    let _lock = CACHE_LOCK.lock().unwrap();

    if let Ok(json) = serde_json::to_string(input) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(CACHE_FILE)
        {
            let _ = writeln!(file, "{}", json);
            println!("[ZK Cache] Input cached locally.");
        } else {
            eprintln!("[ZK Cache] Failed to open cache file.");
        }
    } else {
        eprintln!("[ZK Cache] Failed to serialize input.");
    }
}

/// Loads all cached ZK proof inputs from disk.
pub fn load_cached_inputs() -> Vec<ZkProofInput> {
    let _lock = CACHE_LOCK.lock().unwrap();
    let mut inputs = Vec::new();

    if Path::new(CACHE_FILE).exists() {
        if let Ok(file) = File::open(CACHE_FILE) {
            let reader = BufReader::new(file);
            for line in reader.lines().flatten() {
                if let Ok(input) = serde_json::from_str::<ZkProofInput>(&line) {
                    inputs.push(input);
                }
            }
        }
    }

    inputs
}
