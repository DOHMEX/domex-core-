// src/validator/epoch_oracle.rs

// =========================================================
// epoch_oracle.rs — Domex Epoch Clock and Finality Tracker
// =========================================================
//
// This module defines the global timekeeper for the Domex network.
// It tracks epoch progression, validator finality deadlines, and
// signals when validator proofs must be rotated or finalized.
//
// Epochs are aligned to real-world UTC intervals (e.g. 12s blocks).
// Finality is governed by epoch timers and proof quorum thresholds.
//

use chrono::{Utc, Duration};

/// Epoch parameters
pub const EPOCH_INTERVAL_SECONDS: u64 = 12;  // e.g., 12s finality epochs
pub const FINALITY_TIMEOUT_SECONDS: u64 = 36;  // max 3 epochs delay

/// Represents the Domex epoch clock
#[derive(Debug, Clone)]
pub struct EpochOracle {
    pub current_epoch: u64,
    pub epoch_start_ts: i64, // Unix timestamp
}

impl EpochOracle {
    /// Initialize with the current UTC time
    pub fn new() -> Self {
        let now = Utc::now().timestamp();
        Self {
            current_epoch: 0,
            epoch_start_ts: now,
        }
    }

    /// Returns true if it's time to transition to next epoch
    pub fn is_epoch_expired(&self) -> bool {
        let now = Utc::now().timestamp();
        (now - self.epoch_start_ts) >= EPOCH_INTERVAL_SECONDS as i64
    }

    /// Advances to the next epoch
    pub fn next_epoch(&mut self) {
        self.current_epoch += 1;
        self.epoch_start_ts = Utc::now().timestamp();
    }

    /// Checks if a validator is overdue (missed quorum window)
    pub fn is_validator_late(&self, last_active_epoch: u64) -> bool {
        if self.current_epoch <= last_active_epoch {
            return false;
        }

        let delta = self.current_epoch - last_active_epoch;
        delta * EPOCH_INTERVAL_SECONDS > FINALITY_TIMEOUT_SECONDS
    }

    /// Returns seconds left in current epoch
    pub fn time_remaining(&self) -> i64 {
        let now = Utc::now().timestamp();
        let end_ts = self.epoch_start_ts + EPOCH_INTERVAL_SECONDS as i64;
        (end_ts - now).max(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epoch_oracle_basics() {
        let mut oracle = EpochOracle::new();
        let start_epoch = oracle.current_epoch;

        assert_eq!(oracle.is_epoch_expired(), false);
        assert_eq!(oracle.time_remaining() <= EPOCH_INTERVAL_SECONDS as i64, true);

        // Simulate expiry by manually adjusting timestamp
        oracle.epoch_start_ts -= EPOCH_INTERVAL_SECONDS as i64 + 1;
        assert_eq!(oracle.is_epoch_expired(), true);

        oracle.next_epoch();
        assert_eq!(oracle.current_epoch, start_epoch + 1);
    }

    #[test]
    fn test_validator_lateness() {
        let mut oracle = EpochOracle::new();
        oracle.current_epoch = 10;

        assert_eq!(oracle.is_validator_late(9), false);
        assert_eq!(oracle.is_validator_late(7), true);
    }
}
