use crate::models::Model;
use crate::models::{ledger::LedgerEntryType, NoFlags};
use alloc::borrow::Cow;

use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use serde_with::skip_serializing_none;

use super::{CommonFields, LedgerObject};

/// The `Ticket` object type represents a `Ticket`, which tracks an account sequence number that
/// has been set aside for future use. You can create new tickets with a `TicketCreate` transaction.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Ticket<'a> {
    #[serde(flatten)]
    pub common_fields: CommonFields<'a, NoFlags>,
    /// The account that owns this Ticket.
    pub account: Cow<'a, str>,
    /// A hint indicating which page of the owner directory links to this object, in case the
    /// directory consists of multiple pages.
    pub owner_node: Cow<'a, str>,
    /// The identifying hash of the transaction that most recently modified this object.
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: Cow<'a, str>,
    /// The index of the ledger that contains the transaction that most recently
    /// modified this object.
    pub previous_txn_lgr_seq: u32,
    /// The Sequence Number this Ticket sets aside.
    pub ticket_sequence: u32,
}

impl<'a> Model for Ticket<'a> {}

impl<'a> LedgerObject<NoFlags> for Ticket<'a> {
    fn get_ledger_entry_type(&self) -> LedgerEntryType {
        self.common_fields.get_ledger_entry_type()
    }
}

impl<'a> Ticket<'a> {
    pub fn new(
        index: Option<Cow<'a, str>>,
        ledger_index: Option<Cow<'a, str>>,
        account: Cow<'a, str>,
        owner_node: Cow<'a, str>,
        previous_txn_id: Cow<'a, str>,
        previous_txn_lgr_seq: u32,
        ticket_sequence: u32,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                flags: Vec::new().into(),
                ledger_entry_type: LedgerEntryType::Ticket,
                index,
                ledger_index,
            },
            account,
            owner_node,
            previous_txn_id,
            previous_txn_lgr_seq,
            ticket_sequence,
        }
    }
}

#[cfg(test)]
mod test_serde {
    use super::*;

    #[test]
    fn test_serialize() {
        let ticket = Ticket::new(
            Some(Cow::from("ForTest")),
            None,
            Cow::from("rEhxGqkqPPSxQ3P25J66ft5TwpzV14k2de"),
            Cow::from("0000000000000000"),
            Cow::from("F19AD4577212D3BEACA0F75FE1BA1644F2E854D46E8D62E9C95D18E9708CBFB1"),
            4,
            3,
        );
        let ticket_json = serde_json::to_string(&ticket).unwrap();
        let actual = ticket_json.as_str();
        let expected = r#"{"LedgerEntryType":"Ticket","Flags":0,"index":"ForTest","Account":"rEhxGqkqPPSxQ3P25J66ft5TwpzV14k2de","OwnerNode":"0000000000000000","PreviousTxnID":"F19AD4577212D3BEACA0F75FE1BA1644F2E854D46E8D62E9C95D18E9708CBFB1","PreviousTxnLgrSeq":4,"TicketSequence":3}"#;

        assert_eq!(expected, actual);
    }

    // TODO: test_deserialize
}
