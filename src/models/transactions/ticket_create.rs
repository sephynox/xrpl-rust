use alloc::borrow::Cow;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::amount::XRPAmount;
use crate::models::NoFlags;
use crate::models::{
    model::Model,
    transactions::{Memo, Signer, Transaction, TransactionType},
};

use super::CommonFields;

/// Sets aside one or more sequence numbers as Tickets.
///
/// See TicketCreate:
/// `<https://xrpl.org/ticketcreate.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TicketCreate<'a> {
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(flatten)]
    pub common_fields: CommonFields<'a, NoFlags>,
    /// The custom fields for the TicketCreate model.
    ///
    /// See TicketCreate fields:
    /// `<https://xrpl.org/ticketcreate.html#ticketcreate-fields>`
    pub ticket_count: u32,
}

impl<'a> Model for TicketCreate<'a> {}

impl<'a> Transaction<NoFlags> for TicketCreate<'a> {
    fn get_transaction_type(&self) -> TransactionType {
        self.common_fields.get_transaction_type()
    }
}

impl<'a> TicketCreate<'a> {
    pub fn new(
        account: Cow<'a, str>,
        account_txn_id: Option<Cow<'a, str>>,
        fee: Option<XRPAmount<'a>>,
        last_ledger_sequence: Option<u32>,
        memos: Option<Vec<Memo>>,
        sequence: Option<u32>,
        signers: Option<Vec<Signer<'a>>>,
        source_tag: Option<u32>,
        ticket_sequence: Option<u32>,
        ticket_count: u32,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                account,
                transaction_type: TransactionType::SignerListSet,
                account_txn_id,
                fee,
                flags: None,
                last_ledger_sequence,
                memos,
                sequence,
                signers,
                source_tag,
                ticket_sequence,
            },
            ticket_count,
        }
    }
}

#[cfg(test)]
mod test_serde {
    use super::*;

    #[test]
    fn test_serialize() {
        let default_txn = TicketCreate::new(
            "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn".into(),
            None,
            Some("10".into()),
            None,
            None,
            Some(381),
            None,
            None,
            None,
            10,
        );
        let default_json = r#"{"TransactionType":"TicketCreate","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"10","Sequence":381,"TicketCount":10}"#;

        let txn_as_string = serde_json::to_string(&default_txn).unwrap();
        let txn_json = txn_as_string.as_str();

        assert_eq!(txn_json, default_json);
    }

    #[test]
    fn test_deserialize() {
        let default_txn = TicketCreate::new(
            "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn".into(),
            None,
            Some("10".into()),
            None,
            None,
            Some(381),
            None,
            None,
            None,
            10,
        );
        let default_json = r#"{"TransactionType":"TicketCreate","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"10","Sequence":381,"TicketCount":10}"#;

        let txn_as_obj: TicketCreate = serde_json::from_str(default_json).unwrap();

        assert_eq!(txn_as_obj, default_txn);
    }
}
