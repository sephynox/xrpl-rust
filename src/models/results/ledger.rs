use alloc::{borrow::Cow, vec::Vec};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ledger<'a> {
    pub ledger: LedgerInner<'a>,
    pub ledger_hash: Cow<'a, str>,
    pub ledger_index: u32,
    pub validated: Option<bool>,
    pub queue_data: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerInner<'a> {
    pub account_hash: Cow<'a, str>,
    pub close_flags: u32,
    pub close_time: u32,
    pub close_time_human: Option<Cow<'a, str>>,
    pub close_time_resolution: u32,
    pub closed: bool,
    pub ledger_hash: Cow<'a, str>,
    pub ledger_index: u32,
    pub parent_close_time: u32,
    pub parent_hash: Cow<'a, str>,
    pub total_coins: Cow<'a, str>,
    pub transaction_hash: Cow<'a, str>,
    pub transactions: Option<Vec<Cow<'a, str>>>,
}
