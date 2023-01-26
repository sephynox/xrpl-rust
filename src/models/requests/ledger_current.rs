use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{Model, RequestMethod};

/// The ledger_closed method returns the unique identifiers of
/// the most recently closed ledger. (This ledger is not
/// necessarily validated and immutable yet.)
///
/// See Ledger Closed:
/// `<https://xrpl.org/ledger_closed.html#ledger_closed>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct LedgerCurrent<'a> {
    /// The unique request id.
    pub id: Option<&'a str>,
    /// The request method.
    #[serde(default = "RequestMethod::ledger_current")]
    pub command: RequestMethod,
}

impl Default for LedgerCurrent<'static> {
    fn default() -> Self {
        LedgerCurrent {
            id: None,
            command: RequestMethod::LedgerCurrent,
        }
    }
}

impl Model for LedgerCurrent<'static> {}
