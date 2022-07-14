use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::default_false;

use super::{Currency, PathStep};

/// Required fields for requesting a DepositPreauth if not
/// querying by object ID.
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositPreauthFields<'a> {
    owner: &'a str,
    authorized: &'a str,
}

/// Required fields for requesting a DirectoryNode if not
/// querying by object ID.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryFields<'a> {
    owner: &'a str,
    dir_root: &'a str,
    sub_index: Option<u8>,
}

/// Required fields for requesting a Escrow if not querying
/// by object ID.
#[derive(Debug, Serialize, Deserialize)]
pub struct EscrowFields<'a> {
    owner: &'a str,
    seq: u64,
}

/// A path is an array. Each member of a path is an object that specifies a step on that path.
pub type Path<'a> = Vec<PathStep<'a>>;

/// Required fields for requesting a Escrow if not querying
/// by object ID.
#[derive(Debug, Serialize, Deserialize)]
pub struct OfferFields<'a> {
    account: &'a str,
    seq: u64,
}

/// Required fields for requesting a RippleState.
#[derive(Debug, Serialize, Deserialize)]
pub struct RippleStateFields<'a> {
    account: &'a str,
    currency: &'a str,
}

/// Required fields for requesting a Ticket, if not
/// querying by object ID.
#[derive(Debug, Serialize, Deserialize)]
pub struct TicketFields<'a> {
    owner: &'a str,
    ticket_sequence: u64,
}

/// Format for elements in the `books` array for Subscribe only.
///
/// See Subscribe:
/// `<https://xrpl.org/subscribe.html#subscribe>`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct SubscribeBookFields<'a> {
    taker_gets: Currency,
    taker_pays: Currency,
    taker: &'a str,
    #[serde(default = "default_false")]
    snapshot: Option<bool>,
    #[serde(default = "default_false")]
    both: Option<bool>,
}

/// Format for elements in the `books` array for Unsubscribe only.
///
/// See Unsubscribe:
/// `<https://xrpl.org/unsubscribe.html>`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct UnsubscribeBookFields {
    taker_gets: Currency,
    taker_pays: Currency,
    #[serde(default = "default_false")]
    both: Option<bool>,
}
