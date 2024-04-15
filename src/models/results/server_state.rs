use alloc::borrow::Cow;
use serde::{Deserialize, Serialize};

use crate::models::amount::XRPAmount;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerState<'a> {
    pub state: State<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State<'a> {
    pub build_version: Cow<'a, str>,
    pub network_id: Option<u32>,
    pub validated_ledger: Option<ValidatedLedger<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedLedger<'a> {
    pub base_fee_xrp: XRPAmount<'a>,
    pub close_time: u32,
    pub hash: Cow<'a, str>,
    pub reserve_base: XRPAmount<'a>,
    pub reserve_inc: XRPAmount<'a>,
    pub seq: u32,
}
