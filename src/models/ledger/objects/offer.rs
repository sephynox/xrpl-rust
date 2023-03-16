use crate::_serde::lgr_obj_flags;
use crate::models::ledger::LedgerEntryType;
use crate::models::{Amount, Currency, Model};
use alloc::borrow::Cow;
use alloc::vec::Vec;
use derive_new::new;
use serde::{ser::SerializeMap, Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum_macros::{AsRefStr, Display, EnumIter};

use crate::serialize_with_tag;
use serde_with::skip_serializing_none;

#[derive(
    Debug, Eq, PartialEq, Clone, Serialize_repr, Deserialize_repr, Display, AsRefStr, EnumIter,
)]
#[repr(u32)]
pub enum OfferFlag {
    /// The object was placed as a passive Offer.
    LsfPassive = 0x00010000,
    /// The object was placed as a sell Offer.
    LsfSell = 0x00020000,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Offer<'a> {
    ledger_entry_type: LedgerEntryType,
    #[serde(with = "lgr_obj_flags")]
    flags: Vec<OfferFlag>,
    /// The object ID of a single object to retrieve from the ledger, as a
    /// 64-character (256-bit) hexadecimal string.
    #[serde(rename = "index")]
    pub index: &'a str,
    pub account: &'a str,
    pub book_directory: &'a str,
    pub book_node: &'a str,
    pub owner_node: &'a str,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: &'a str,
    pub previous_txn_lgr_seq: u32,
    pub sequence: u32,
    pub taker_gets: Amount,
    pub taker_pays: Amount,
    pub expiration: Option<u32>,
}

impl<'a> Default for Offer<'a> {
    fn default() -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::Offer,
            flags: Default::default(),
            index: Default::default(),
            account: Default::default(),
            book_directory: Default::default(),
            book_node: Default::default(),
            owner_node: Default::default(),
            previous_txn_id: Default::default(),
            previous_txn_lgr_seq: Default::default(),
            sequence: Default::default(),
            taker_gets: Default::default(),
            taker_pays: Default::default(),
            expiration: Default::default(),
        }
    }
}

impl<'a> Model for Offer<'a> {}

impl<'a> Offer<'a> {
    pub fn new(
        flags: Vec<OfferFlag>,
        index: &'a str,
        account: &'a str,
        book_directory: &'a str,
        book_node: &'a str,
        owner_node: &'a str,
        previous_txn_id: &'a str,
        previous_txn_lgr_seq: u32,
        sequence: u32,
        taker_gets: Amount,
        taker_pays: Amount,
        expiration: Option<u32>,
    ) -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::Offer,
            flags,
            index,
            account,
            book_directory,
            book_node,
            owner_node,
            previous_txn_id,
            previous_txn_lgr_seq,
            sequence,
            taker_gets,
            taker_pays,
            expiration,
        }
    }
}

#[cfg(test)]
mod test_serde {
    use super::*;
    use alloc::borrow::Cow;
    use alloc::vec;

    #[test]
    fn test_serialize() {
        let offer = Offer::new(
            vec![OfferFlag::LsfSell],
            "96F76F27D8A327FC48753167EC04A46AA0E382E6F57F32FD12274144D00F1797",
            "rBqb89MRQJnMPq8wTwEbtz4kvxrEDfcYvt",
            "ACC27DE91DBA86FC509069EAF4BC511D73128B780F2E54BF5E07A369E2446000",
            "0000000000000000",
            "0000000000000000",
            "F0AB71E777B2DA54B86231E19B82554EF1F8211F92ECA473121C655BFC5329BF",
            14524914,
            866,
            Amount::IssuedCurrency {
                currency: Cow::from("XAG"),
                issuer: Cow::from("r9Dr5xwkeLegBeXq6ujinjSBLQzQ1zQGjH"),
                value: Cow::from("37"),
            },
            Amount::Xrp(Cow::from("79550000000")),
            None,
        );
        let offer_json = serde_json::to_string(&offer).unwrap();
        let actual = offer_json.as_str();
        let expected = r#"{"LedgerEntryType":"Offer","Flags":131072,"index":"96F76F27D8A327FC48753167EC04A46AA0E382E6F57F32FD12274144D00F1797","Account":"rBqb89MRQJnMPq8wTwEbtz4kvxrEDfcYvt","BookDirectory":"ACC27DE91DBA86FC509069EAF4BC511D73128B780F2E54BF5E07A369E2446000","BookNode":"0000000000000000","OwnerNode":"0000000000000000","PreviousTxnID":"F0AB71E777B2DA54B86231E19B82554EF1F8211F92ECA473121C655BFC5329BF","PreviousTxnLgrSeq":14524914,"Sequence":866,"TakerGets":{"currency":"XAG","issuer":"r9Dr5xwkeLegBeXq6ujinjSBLQzQ1zQGjH","value":"37"},"TakerPays":"79550000000"}"#;

        assert_eq!(expected, actual);
    }
}
