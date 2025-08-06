// ==========================================================    
// governance_dao.rs — Domex Governance DAO Voting System    
// ==========================================================    
//    
// Handles proposal creation, validator voting, and result evaluation.    
// Used for token onboarding, parameter updates, or DAO-level resolutions.    
//    
    
use std::collections::HashMap;    
use std::time::{SystemTime, UNIX_EPOCH};    
    
use crate::validator::validator_identity::ValidatorId;    
    
/// Governance proposal types    
#[derive(Debug, Clone)]    
pub enum ProposalType {    
    TokenOnboarding { token_symbol: String, metadata_uri: String },    
    UpdateParameter { key: String, new_value: String },    
    ValidatorPolicyChange { policy_name: String, action: String },    
}    
    
/// Governance proposal structure    
#[derive(Debug, Clone)]    
pub struct Proposal {    
    pub id: u64,    
    pub proposer: ValidatorId,    
    pub proposal_type: ProposalType,    
    pub description: String,    
    pub created_at: u64,    
    pub vote_deadline: u64,    
    pub yes_votes: u64,    
    pub no_votes: u64,    
    pub executed: bool,    
}    
    
/// Global DAO state    
#[derive(Default)]    
pub struct GovernanceDAO {    
    pub proposals: HashMap<u64, Proposal>,    
    pub votes: HashMap<u64, HashMap<ValidatorId, bool>>, // Proposal ID → Voter → Yes/No    
    pub next_id: u64,    
    pub voting_period_secs: u64,    
}    
    
impl GovernanceDAO {    
    pub fn new(voting_period_secs: u64) -> Self {    
        Self {    
            proposals: HashMap::new(),    
            votes: HashMap::new(),    
            next_id: 0,    
            voting_period_secs,    
        }    
    }    
    
    /// Create new proposal    
    pub fn create_proposal(    
        &mut self,    
        proposer: ValidatorId,    
        proposal_type: ProposalType,    
        description: String,    
    ) -> u64 {    
        let now = current_timestamp();    
        let id = self.next_id;    
        self.next_id += 1;    
    
        let proposal = Proposal {    
            id,    
            proposer,    
            proposal_type,    
            description,    
            created_at: now,    
            vote_deadline: now + self.voting_period_secs,    
            yes_votes: 0,    
            no_votes: 0,    
            executed: false,    
        };    
    
        self.proposals.insert(id, proposal);    
        id    
    }    
    
    /// Cast vote for a proposal    
    pub fn vote(&mut self, validator: ValidatorId, proposal_id: u64, approve: bool) -> Result<(), String> {    
        if let Some(proposal) = self.proposals.get_mut(&proposal_id) {    
            if current_timestamp() > proposal.vote_deadline {    
                return Err("Voting period expired".into());    
            }    
    
            let validator_votes = self.votes.entry(proposal_id).or_default();    
            if validator_votes.contains_key(&validator) {    
                return Err("Validator already voted".into());    
            }    
    
            validator_votes.insert(validator, approve);    
            if approve {    
                proposal.yes_votes += 1;    
            } else {    
                proposal.no_votes += 1;    
            }    
    
            Ok(())    
        } else {    
            Err("Proposal not found".into())    
        }    
    }    
    
    /// Finalize and mark proposal as executed if passed    
    pub fn execute_proposal(&mut self, proposal_id: u64, quorum: u64) -> Result<bool, String> {    
        if let Some(proposal) = self.proposals.get_mut(&proposal_id) {    
            if proposal.executed {    
                return Err("Proposal already executed".into());    
            }    
            if current_timestamp() < proposal.vote_deadline {    
                return Err("Voting still active".into());    
            }    
    
            let total_votes = proposal.yes_votes + proposal.no_votes;    
            if total_votes < quorum {    
                return Err("Quorum not met".into());    
            }    
    
            if proposal.yes_votes > proposal.no_votes {    
                proposal.executed = true;    
                Ok(true)    
            } else {    
                proposal.executed = true;    
                Ok(false)    
            }    
        } else {    
            Err("Proposal not found".into())    
        }    
    }    
}    
    
/// Utility to get current timestamp in seconds    
fn current_timestamp() -> u64 {    
    SystemTime::now()    
        .duration_since(UNIX_EPOCH)    
        .unwrap()    
        .as_secs()    
}
