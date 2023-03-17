use crate::models::ledger::LedgerEntryType;
use crate::models::Model;

use serde::{Deserialize, Serialize};

use serde_with::skip_serializing_none;

/// The `Ticket` object type represents a `Ticket`, which tracks an account sequence number that
/// has been set aside for future use. You can create new tickets with a `TicketCreate` transaction.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Ticket<'a> {
    /// The value 0x0054, mapped to the string Ticket, indicates that this object
    /// is a Ticket object.
    pub ledger_entry_type: LedgerEntryType,
    /// A bit-map of boolean flags enabled for this object. Currently, the protocol defines
    /// no flags for Ticket objects. The value is always 0.
    pub flags: u32,
    /// The object ID of a single object to retrieve from the ledger, as a
    /// 64-character (256-bit) hexadecimal string.
    #[serde(rename = "index")]
    pub index: &'a str,
    /// The account that owns this Ticket.
    pub account: &'a str,
    /// A hint indicating which page of the owner directory links to this object, in case the
    /// directory consists of multiple pages.
    pub owner_node: &'a str,
    /// The identifying hash of the transaction that most recently modified this object.
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: &'a str,
    /// The index of the ledger that contains the transaction that most recently
    /// modified this object.
    pub previous_txn_lgr_seq: u32,
    /// The Sequence Number this Ticket sets aside.
    pub ticket_sequence: u32,
}

impl<'a> Default for Ticket<'a> {
    fn default() -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::Ticket,
            flags: Default::default(),
            index: Default::default(),
            account: Default::default(),
            owner_node: Default::default(),
            previous_txn_id: Default::default(),
            previous_txn_lgr_seq: Default::default(),
            ticket_sequence: Default::default(),
        }
    }
}

impl<'a> Model for Ticket<'a> {}

impl<'a> Ticket<'a> {
    pub fn new(
        index: &'a str,
        account: &'a str,
        owner_node: &'a str,
        previous_txn_id: &'a str,
        previous_txn_lgr_seq: u32,
        ticket_sequence: u32,
    ) -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::Ticket,
            flags: 0,
            index,
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
            "ForTest",
            "rEhxGqkqPPSxQ3P25J66ft5TwpzV14k2de",
            "0000000000000000",
            "F19AD4577212D3BEACA0F75FE1BA1644F2E854D46E8D62E9C95D18E9708CBFB1",
            4,
            3,
        );
        let ticket_json = serde_json::to_string(&ticket).unwrap();
        let actual = ticket_json.as_str();
        let expected = r#"{"LedgerEntryType":"Ticket","Flags":0,"index":"ForTest","Account":"rEhxGqkqPPSxQ3P25J66ft5TwpzV14k2de","OwnerNode":"0000000000000000","PreviousTxnID":"F19AD4577212D3BEACA0F75FE1BA1644F2E854D46E8D62E9C95D18E9708CBFB1","PreviousTxnLgrSeq":4,"TicketSequence":3}"#;

        assert_eq!(expected, actual);
    }
}
