use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{request_fields::SubscribeBookFields, Model, RequestMethod, StreamParameter};

/// The unsubscribe command tells the server to stop
/// sending messages for a particular subscription or set
/// of subscriptions.
///
/// Note: WebSocket API only.
///
/// See Unsubscribe:
/// `<https://xrpl.org/unsubscribe.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Unsubscribe<'a> {
    /// The unique request id.
    pub id: Option<&'a str>,
    /// Array of objects defining order books to unsubscribe
    /// from, as explained below.
    // TODO: USE `UnsubscribeBookFields` AS SOON AS LIFETIME ISSUES ARE FIXED
    pub books: Option<Vec<SubscribeBookFields<'a>>>,
    /// Array of string names of generic streams to unsubscribe
    /// from, including ledger, server, transactions,
    /// and transactions_proposed.
    pub streams: Option<Vec<StreamParameter>>,
    /// Array of unique account addresses to stop receiving updates
    /// for, in the XRP Ledger's base58 format. (This only stops
    /// those messages if you previously subscribed to those accounts
    /// specifically. You cannot use this to filter accounts out of
    /// the general transactions stream.)
    pub accounts: Option<Vec<&'a str>>,
    /// Like accounts, but for accounts_proposed subscriptions that
    /// included not-yet-validated transactions.
    pub accounts_proposed: Option<Vec<&'a str>>,
    // TODO Lifetime issue
    #[serde(skip_serializing)]
    pub broken: Option<&'a str>,
    /// The request method.
    #[serde(default = "RequestMethod::unsubscribe")]
    pub command: RequestMethod,
}

impl Default for Unsubscribe<'static> {
    fn default() -> Self {
        Unsubscribe {
            id: None,
            books: None,
            streams: None,
            accounts: None,
            accounts_proposed: None,
            broken: None,
            command: RequestMethod::Unsubscribe,
        }
    }
}

impl Model for Unsubscribe<'static> {}
