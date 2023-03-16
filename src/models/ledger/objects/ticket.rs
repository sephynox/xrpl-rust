use crate::models::ledger::LedgerEntryType;
use crate::models::{Amount, Currency, Model};
use alloc::borrow::Cow;
use alloc::vec::Vec;
use derive_new::new;
use serde::{ser::SerializeMap, Deserialize, Serialize};

use crate::serialize_with_tag;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Ticket<'a> {
    ledger_entry_type: LedgerEntryType,
    flags: u32,
    /// The object ID of a single object to retrieve from the ledger, as a
    /// 64-character (256-bit) hexadecimal string.
    #[serde(rename = "index")]
    pub index: &'a str,
    pub account: &'a str,
    pub owner_node: &'a str,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: &'a str,
    pub previous_txn_lgr_seq: u32,
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
