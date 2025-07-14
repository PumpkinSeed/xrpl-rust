use alloc::vec::Vec;
use objects::LedgerEntry;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod objects;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum LedgerVersionMap {
    Default(Ledger),
    V1(LedgerV1),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Ledger {
    #[serde(flatten)]
    pub base: BaseLedger,
    pub ledger_index: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct LedgerV1 {
    #[serde(flatten)]
    pub base: BaseLedger,
    pub ledger_index: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct BaseLedger {
    pub account_hash: String,
    pub account_state: Option<Vec<LedgerEntry>>,
    pub close_flags: u32,
    pub close_time: u64,
    pub close_time_human: String,
    pub close_time_resolution: u32,
    pub close_time_iso: String,
    pub closed: bool,
    pub ledger_hash: String,
    pub parent_close_time: u64,
    pub parent_hash: String,
    pub total_coins: String,
    pub transaction_hash: String,
    pub transactions: Option<Vec<TransactionWithMetadata>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct TransactionWithMetadata {
    pub hash: String,
    pub metadata: Option<Value>, // TODO: Replace with actual metadata as soon as it's implemented
}
