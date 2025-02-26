use alloc::borrow::Cow;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{currency::Currency, requests::RequestMethod, Model};

use super::{CommonFields, LedgerIndex, LookupByLedgerRequest, Request};

/// The ripple_path_find method is a simpl<'a>ified version of
/// the path_find method that provides a single response with
/// a payment path you can use right away. It is available in
/// both the WebSocket and JSON-RPC APIs. However, the
/// results tend to become outdated as time passes. Instead of
/// making multiple calls to stay updated, you should instead
/// use the path_find method to subscribe to continued updates
/// where possible.
///
/// Although the rippled server tries to find the cheapest path
/// or combination of paths for making a payment, it is not
/// guaranteed that the paths returned by this method are, in
/// fact, the best paths.
///
/// See Ripple Path Find:
/// `<https://xrpl.org/ripple_path_find.html#ripple_path_find>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct RipplePathFind<'a> {
    /// The common fields shared by all requests.
    #[serde(flatten)]
    pub common_fields: CommonFields<'a>,
    /// Unique address of the account that would receive funds
    /// in a transaction.
    pub destination_account: Cow<'a, str>,
    /// Currency Amount that the destination account would
    /// receive in a transaction. Special case: New in: rippled 0.30.0
    /// You can specify "-1" (for XRP) or provide -1 as the contents
    /// of the value field (for non-XRP currencies). This requests a
    /// path to deliver as much as possible, while spending no more
    /// than the amount specified in send_max (if provided).
    pub destination_amount: Currency<'a>,
    /// Unique address of the account that would send funds
    /// in a transaction.
    pub source_account: Cow<'a, str>,
    /// The unique identifier of a ledger.
    #[serde(flatten)]
    pub ledger_lookup: Option<LookupByLedgerRequest<'a>>,
    /// Currency Amount that would be spent in the transaction.
    /// Cannot be used with source_currencies.
    pub send_max: Option<Currency<'a>>,
    /// Array of currencies that the source account might want
    /// to spend. Each entry in the array should be a JSON object
    /// with a mandatory currency field and optional issuer field,
    /// like how currency amounts are specified. Cannot contain
    /// more than 18 source currencies. By default, uses all source
    /// currencies available up to a maximum of 88 different
    /// currency/issuer pairs.
    pub source_currencies: Option<Vec<Currency<'a>>>,
}

impl<'a> Model for RipplePathFind<'a> {}

impl<'a> Request<'a> for RipplePathFind<'a> {
    fn get_common_fields(&self) -> &CommonFields<'a> {
        &self.common_fields
    }

    fn get_common_fields_mut(&mut self) -> &mut CommonFields<'a> {
        &mut self.common_fields
    }
}

impl<'a> RipplePathFind<'a> {
    pub fn new(
        id: Option<Cow<'a, str>>,
        destination_account: Cow<'a, str>,
        destination_amount: Currency<'a>,
        source_account: Cow<'a, str>,
        ledger_hash: Option<Cow<'a, str>>,
        ledger_index: Option<LedgerIndex<'a>>,
        send_max: Option<Currency<'a>>,
        source_currencies: Option<Vec<Currency<'a>>>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                command: RequestMethod::RipplePathFind,
                id,
            },
            destination_account,
            destination_amount,
            source_account,
            ledger_lookup: Some(LookupByLedgerRequest {
                ledger_hash,
                ledger_index,
            }),
            send_max,
            source_currencies,
        }
    }
}
