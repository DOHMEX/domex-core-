// ==========================================================
// zk_summit.rs — Domex Client ZK Activity Dashboard (Read-Only)
// ==========================================================
//
// Summarizes local proof activity, delegator usage, and onboarding flow.
// Used for UIs, dashboards, or client-side analytics.
//
// This is optional — no critical logic happens here.

use crate::zk_client::cold_deposit::ColdDeposit;
use crate::types::common::{Token, ZkIdentity};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ZkActivitySummary {
    pub total_deposits: usize,
    pub claimable_deposits: usize,
    pub claimed_deposits: usize,
    pub autovault_enabled: usize,
    pub tokens_seen: Vec<Token>,
}

pub struct ZkSummit {
    pub deposits: Vec<ColdDeposit>,
}

impl ZkSummit {
    pub fn new(deposits: Vec<ColdDeposit>) -> Self {
        Self { deposits }
    }

    /// Generates a high-level summary of onboarding and deposit flow
    pub fn summarize(&self) -> ZkActivitySummary {
        let mut tokens_seen = HashMap::new();
        let mut claimable = 0;
        let mut claimed = 0;
        let mut autovault = 0;

        for dep in &self.deposits {
            tokens_seen.insert(dep.token.clone(), true);
            if dep.claimed {
                claimed += 1;
            } else {
                claimable += 1;
            }
            if dep.autovault {
                autovault += 1;
            }
        }

        ZkActivitySummary {
            total_deposits: self.deposits.len(),
            claimable_deposits: claimable,
            claimed_deposits: claimed,
            autovault_enabled: autovault,
            tokens_seen: tokens_seen.keys().cloned().collect(),
        }
    }
}
