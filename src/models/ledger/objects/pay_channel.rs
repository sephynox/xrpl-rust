use crate::models::ledger::LedgerEntryType;
use crate::models::{Amount, Model};

use serde::{Deserialize, Serialize};

use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PayChannel<'a> {
    ledger_entry_type: LedgerEntryType,
    flags: u32,
    /// The object ID of a single object to retrieve from the ledger, as a
    /// 64-character (256-bit) hexadecimal string.
    #[serde(rename = "index")]
    pub index: &'a str,
    pub account: &'a str,
    pub amount: Amount,
    pub balance: Amount,
    pub destination: &'a str,
    pub owner_node: &'a str,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: &'a str,
    pub previous_txn_lgr_seq: u32,
    pub public_key: &'a str,
    pub settle_delay: u32,
    pub cancel_after: Option<u32>,
    pub destination_tag: Option<u32>,
    pub destination_node: Option<&'a str>,
    pub expiration: Option<u32>,
    pub source_tag: Option<u32>,
}

impl<'a> Default for PayChannel<'a> {
    fn default() -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::PayChannel,
            flags: Default::default(),
            index: Default::default(),
            account: Default::default(),
            amount: Default::default(),
            balance: Default::default(),
            destination: Default::default(),
            owner_node: Default::default(),
            previous_txn_id: Default::default(),
            previous_txn_lgr_seq: Default::default(),
            public_key: Default::default(),
            settle_delay: Default::default(),
            cancel_after: Default::default(),
            destination_tag: Default::default(),
            destination_node: Default::default(),
            expiration: Default::default(),
            source_tag: Default::default(),
        }
    }
}

impl<'a> Model for PayChannel<'a> {}

impl<'a> PayChannel<'a> {
    pub fn new(
        index: &'a str,
        account: &'a str,
        amount: Amount,
        balance: Amount,
        destination: &'a str,
        owner_node: &'a str,
        previous_txn_id: &'a str,
        previous_txn_lgr_seq: u32,
        public_key: &'a str,
        settle_delay: u32,
        cancel_after: Option<u32>,
        destination_tag: Option<u32>,
        destination_node: Option<&'a str>,
        expiration: Option<u32>,
        source_tag: Option<u32>,
    ) -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::PayChannel,
            flags: 0,
            index,
            account,
            amount,
            balance,
            destination,
            owner_node,
            previous_txn_id,
            previous_txn_lgr_seq,
            public_key,
            settle_delay,
            cancel_after,
            destination_tag,
            destination_node,
            expiration,
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
        let pay_channel = PayChannel::new(
            "96F76F27D8A327FC48753167EC04A46AA0E382E6F57F32FD12274144D00F1797",
            "rBqb89MRQJnMPq8wTwEbtz4kvxrEDfcYvt",
            Amount::Xrp(Cow::from("4325800")),
            Amount::Xrp(Cow::from("2323423")),
            "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
            "0000000000000000",
            "F0AB71E777B2DA54B86231E19B82554EF1F8211F92ECA473121C655BFC5329BF",
            14524914,
            "32D2471DB72B27E3310F355BB33E339BF26F8392D5A93D3BC0FC3B566612DA0F0A",
            3600,
            Some(536891313),
            Some(1002341),
            Some("0000000000000000"),
            Some(536027313),
            Some(0),
        );
        let pay_channel_json = serde_json::to_string(&pay_channel).unwrap();
        let actual = pay_channel_json.as_str();
        let expected = r#"{"LedgerEntryType":"PayChannel","Flags":0,"index":"96F76F27D8A327FC48753167EC04A46AA0E382E6F57F32FD12274144D00F1797","Account":"rBqb89MRQJnMPq8wTwEbtz4kvxrEDfcYvt","Amount":"4325800","Balance":"2323423","Destination":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","OwnerNode":"0000000000000000","PreviousTxnID":"F0AB71E777B2DA54B86231E19B82554EF1F8211F92ECA473121C655BFC5329BF","PreviousTxnLgrSeq":14524914,"PublicKey":"32D2471DB72B27E3310F355BB33E339BF26F8392D5A93D3BC0FC3B566612DA0F0A","SettleDelay":3600,"CancelAfter":536891313,"DestinationTag":1002341,"DestinationNode":"0000000000000000","Expiration":536027313,"SourceTag":0}"#;

        assert_eq!(expected, actual);
    }
}
