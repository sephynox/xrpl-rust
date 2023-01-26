use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{Model, RequestMethod};

/// The transaction_entry method retrieves information on a
/// single transaction from a specific ledger version.
/// (The tx method, by contrast, searches all ledgers for
/// the specified transaction. We recommend using that
/// method instead.)
///
/// See Transaction Entry:
/// `<https://xrpl.org/transaction_entry.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionEntry<'a> {
    /// Unique hash of the transaction you are looking up.
    pub tx_hash: &'a str,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// A 20-byte hex string for the ledger version to use.
    pub ledger_hash: Option<&'a str>,
    /// The ledger index of the ledger to use, or a shortcut
    /// string to choose a ledger automatically.
    pub ledger_index: Option<&'a str>,
    /// The request method.
    #[serde(default = "RequestMethod::transaction_entry")]
    pub command: RequestMethod,
}

impl Default for TransactionEntry<'static> {
    fn default() -> Self {
        TransactionEntry {
            tx_hash: "",
            id: None,
            ledger_hash: None,
            ledger_index: None,
            command: RequestMethod::TransactionEntry,
        }
    }
}

impl Model for TransactionEntry<'static> {}