// ==========================================================
// btc_fee_estimator.rs — Domex BTC Fee Estimator (Sat/vByte)
// ==========================================================
//
// Estimates optimal Bitcoin fee rate (sats per vByte) for withdrawals.
// Can use static fallback or external APIs (e.g., mempool.space).
// Vault builders use this to set transaction fees.

use std::time::{Duration, Instant};
use std::sync::Mutex;

// Optional: Add actual HTTP fetch logic if internet access is available.
// For now, we simulate with static fallback.

lazy_static::lazy_static! {
    static ref FEE_CACHE: Mutex<(f64, Instant)> = Mutex::new((12.5, Instant::now()));
}

/// Returns estimated fee rate in satoshis per vByte
pub fn estimate_fee_rate() -> f64 {
    let mut cache = FEE_CACHE.lock().unwrap();
    let now = Instant::now();

    // If cached fee is older than 5 minutes, simulate a new fetch
    if now.duration_since(cache.1) > Duration::from_secs(300) {
        // Simulated logic (e.g., could fetch from mempool.space or Electrum)
        let simulated_rate = 14.0 + (rand::random::<f64>() % 4.0); // e.g., 14.0 to 18.0 sats/vB
        *cache = (simulated_rate, now);
    }

    cache.0
}
