use alloc::{borrow::Cow, vec::Vec};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{NoFlags, XChainBridge};

use super::{CommonFields, XChainClaimProofSig};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct XChainOwnedCreateAccountClaimID<'a> {
    #[serde(flatten)]
    pub common_fields: CommonFields<'a, NoFlags>,
    pub account: Cow<'a, str>,
    #[serde(rename = "XChainAccountCreateCount")]
    pub xchain_account_create_count: u64,
    #[serde(rename = "XChainBridge")]
    pub xchain_bridge: XChainBridge<'a>,
    pub xchain_create_account_attestations: Vec<XChainClaimProofSig<'a>>,
}
