use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{Model, RequestMethod};

/// The fee command reports the current state of the open-ledger
/// requirements for the transaction cost. This requires the
/// FeeEscalation amendment to be enabled. This is a public
/// command available to unprivileged users.
///
/// See Fee:
/// `<https://xrpl.org/fee.html#fee>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Fee<'a> {
    /// The unique request id.
    pub id: Option<&'a str>,
    /// The request method.
    #[serde(default = "RequestMethod::fee")]
    pub command: RequestMethod,
}

impl Default for Fee<'static> {
    fn default() -> Self {
        Fee {
            id: None,
            command: RequestMethod::Fee,
        }
    }
}

impl Model for Fee<'static> {}