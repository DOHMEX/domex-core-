// types/merkle.rs
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MerkleDelta {
    pub identity: String,
    pub token: String,
    pub before: i64,
    pub after: i64,
}
