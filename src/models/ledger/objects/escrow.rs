use crate::models::ledger::LedgerEntryType;
use crate::models::{Amount, Model};
use serde::{Deserialize, Serialize};

use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Escrow<'a> {
    ledger_entry_type: LedgerEntryType,
    account: &'a str,
    amount: Amount,
    cancel_after: Option<u32>,
    condition: Option<&'a str>,
    destination: &'a str,
    destination_node: Option<&'a str>,
    destination_tag: Option<u32>,
    finish_after: Option<u32>,
    flags: u32,
    /// The object ID of a single object to retrieve from the ledger, as a
    /// 64-character (256-bit) hexadecimal string.
    #[serde(rename = "index")]
    index: &'a str,
    owner_node: &'a str,
    #[serde(rename = "PreviousTxnID")]
    previous_txn_id: &'a str,
    previous_txn_lgr_seq: u32,
    source_tag: Option<u32>,
}

impl<'a> Default for Escrow<'a> {
    fn default() -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::Escrow,
            account: Default::default(),
            amount: Default::default(),
            cancel_after: Default::default(),
            condition: Default::default(),
            destination: Default::default(),
            destination_node: Default::default(),
            destination_tag: Default::default(),
            finish_after: Default::default(),
            flags: Default::default(),
            index: Default::default(),
            owner_node: Default::default(),
            previous_txn_id: Default::default(),
            previous_txn_lgr_seq: Default::default(),
            source_tag: Default::default(),
        }
    }
}

impl<'a> Model for Escrow<'a> {}

impl<'a> Escrow<'a> {
    pub fn new(
        account: &'a str,
        amount: Amount,
        destination: &'a str,
        flags: u32,
        index: &'a str,
        owner_node: &'a str,
        previous_txn_id: &'a str,
        previous_txn_lgr_seq: u32,
        cancel_after: Option<u32>,
        condition: Option<&'a str>,
        destination_node: Option<&'a str>,
        destination_tag: Option<u32>,
        finish_after: Option<u32>,
        source_tag: Option<u32>,
    ) -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::Escrow,
            account,
            amount,
            cancel_after,
            condition,
            destination,
            destination_node,
            destination_tag,
            finish_after,
            flags,
            index,
            owner_node,
            previous_txn_id,
            previous_txn_lgr_seq,
            source_tag,
        }
    }
}

#[cfg(test)]
mod test_serde {
    use super::*;
    use alloc::borrow::Cow;

    #[test]
    fn test_serialize() {
        let escrow = Escrow::new(
            "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
            Amount::Xrp(Cow::from("10000")),
            "ra5nK24KXen9AHvsdFTKHSANinZseWnPcX",
            0,
            "DC5F3851D8A1AB622F957761E5963BC5BD439D5C24AC6AD7AC4523F0640244AC",
            "0000000000000000",
            "C44F2EB84196B9AD820313DBEBA6316A15C9A2D35787579ED172B87A30131DA7",
            28991004,
            Some(545440232),
            Some("A0258020A82A88B2DF843A54F58772E4A3861866ECDB4157645DD9AE528C1D3AEEDABAB6810120"),
            Some("0000000000000000"),
            Some(23480),
            Some(545354132),
            Some(11747),
        );
        let escrow_json = serde_json::to_string(&escrow).unwrap();
        let actual = escrow_json.as_str();
        let expected = r#"{"LedgerEntryType":"Escrow","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Amount":"10000","CancelAfter":545440232,"Condition":"A0258020A82A88B2DF843A54F58772E4A3861866ECDB4157645DD9AE528C1D3AEEDABAB6810120","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","DestinationNode":"0000000000000000","DestinationTag":23480,"FinishAfter":545354132,"Flags":0,"index":"DC5F3851D8A1AB622F957761E5963BC5BD439D5C24AC6AD7AC4523F0640244AC","OwnerNode":"0000000000000000","PreviousTxnID":"C44F2EB84196B9AD820313DBEBA6316A15C9A2D35787579ED172B87A30131DA7","PreviousTxnLgrSeq":28991004,"SourceTag":11747}"#;

        assert_eq!(expected, actual);
    }
}
