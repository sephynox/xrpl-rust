use crate::models::ledger::LedgerEntryType;
use crate::models::{Amount, Currency, Model};
use alloc::borrow::Cow;
use alloc::vec::Vec;
use derive_new::new;
use serde::{ser::SerializeMap, Deserialize, Serialize};

use crate::serialize_with_tag;
use serde_with::skip_serializing_none;

/// A Check object describes a check, similar to a paper personal check, which can be cashed by its
/// destination to get money from its sender.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Check<'a> {
    /// The value 0x0043, mapped to the string Check, indicates that this object is a Check object.
    ledger_entry_type: LedgerEntryType,
    /// The sender of the Check. Cashing the Check debits this address's balance.
    account: &'a str,
    /// The intended recipient of the Check. Only this address can cash the Check, using a `CheckCash`
    /// transaction.
    destination: &'a str,
    /// A hint indicating which page of the destination's owner directory links to this object, in
    /// case the directory consists of multiple pages.
    destination_node: Option<&'a str>,
    /// An arbitrary tag to further specify the destination for this `Check`, such as a hosted
    /// recipient at the destination address.
    destination_tag: Option<u32>,
    /// Indicates the time after which this `Check` is considered expired.
    expiration: Option<u32>,
    /// A bit-map of boolean flags enabled for this object. Currently, the protocol defines no flags
    /// for `Check` objects. The value is always 0.
    flags: u32,
    #[serde(rename = "index")]
    index: &'a str,
    /// Arbitrary 256-bit hash provided by the sender as a specific reason or identifier for this Check.
    #[serde(rename = "InvoiceID")]
    invoice_id: Option<&'a str>,
    /// A hint indicating which page of the sender's owner directory links to this object, in case
    /// the directory consists of multiple pages.
    owner_node: &'a str,
    /// The identifying hash of the transaction that most recently modified this object.
    #[serde(rename = "PreviousTxnID")]
    previous_txn_id: &'a str,
    /// The index of the ledger that contains the transaction that most recently modified this object.
    previous_txn_lgr_seq: u32,
    /// The maximum amount of currency this Check can debit the sender. If the Check is successfully
    /// cashed, the destination is credited in the same currency for up to this amount.
    send_max: Amount,
    /// The sequence number of the CheckCreate transaction that created this check.
    sequence: u32,
    /// An arbitrary tag to further specify the source for this Check, such as a hosted recipient at
    /// the sender's address.
    source_tag: Option<u32>,
}

impl<'a> Default for Check<'a> {
    fn default() -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::Check,
            account: Default::default(),
            destination: Default::default(),
            destination_node: Default::default(),
            destination_tag: Default::default(),
            expiration: Default::default(),
            flags: Default::default(),
            index: Default::default(),
            invoice_id: Default::default(),
            owner_node: Default::default(),
            previous_txn_id: Default::default(),
            previous_txn_lgr_seq: Default::default(),
            send_max: Default::default(),
            sequence: Default::default(),
            source_tag: Default::default(),
        }
    }
}

impl<'a> Model for Check<'a> {}

impl<'a> Check<'a> {
    pub fn new(
        account: &'a str,
        destination: &'a str,
        flags: u32,
        index: &'a str,
        owner_node: &'a str,
        previous_txn_id: &'a str,
        previous_txn_lgr_seq: u32,
        send_max: Amount,
        sequence: u32,
        destination_node: Option<&'a str>,
        destination_tag: Option<u32>,
        expiration: Option<u32>,
        invoice_id: Option<&'a str>,
        source_tag: Option<u32>,
    ) -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::Check,
            account,
            destination,
            destination_node,
            destination_tag,
            expiration,
            flags,
            index,
            invoice_id,
            owner_node,
            previous_txn_id,
            previous_txn_lgr_seq,
            send_max,
            sequence,
            source_tag,
        }
    }
}

#[cfg(test)]
mod test_serde {
    use super::*;

    #[test]
    fn test_serialize() {
        let check = Check::new(
            "rUn84CUYbNjRoTQ6mSW7BVJPSVJNLb1QLo",
            "rfkE1aSy9G8Upk4JssnwBxhEv5p4mn2KTy",
            0,
            "49647F0D748DC3FE26BDACBC57F251AADEFFF391403EC9BF87C97F67E9977FB0",
            "0000000000000000",
            "5463C6E08862A1FAE5EDAC12D70ADB16546A1F674930521295BC082494B62924",
            6,
            Amount::Xrp(Cow::from("100000000")),
            2,
            Some("0000000000000000"),
            Some(1),
            Some(570113521),
            Some("46060241FABCF692D4D934BA2A6C4427CD4279083E38C77CBE642243E43BE291"),
            None,
        );
        let check_json = serde_json::to_string(&check).unwrap();
        let actual = check_json.as_str();
        let expected = r#"{"LedgerEntryType":"Check","Account":"rUn84CUYbNjRoTQ6mSW7BVJPSVJNLb1QLo","Destination":"rfkE1aSy9G8Upk4JssnwBxhEv5p4mn2KTy","DestinationNode":"0000000000000000","DestinationTag":1,"Expiration":570113521,"Flags":0,"index":"49647F0D748DC3FE26BDACBC57F251AADEFFF391403EC9BF87C97F67E9977FB0","InvoiceID":"46060241FABCF692D4D934BA2A6C4427CD4279083E38C77CBE642243E43BE291","OwnerNode":"0000000000000000","PreviousTxnID":"5463C6E08862A1FAE5EDAC12D70ADB16546A1F674930521295BC082494B62924","PreviousTxnLgrSeq":6,"SendMax":"100000000","Sequence":2}"#;

        assert_eq!(expected, actual)
    }

    // TODO: test_deserialize
}
