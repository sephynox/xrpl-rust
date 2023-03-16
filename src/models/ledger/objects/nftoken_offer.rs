use crate::_serde::lgr_obj_flags;
use crate::models::ledger::LedgerEntryType;
use crate::models::{Amount, Currency, Model};
use alloc::borrow::Cow;
use alloc::vec::Vec;
use derive_new::new;
use serde::{ser::SerializeMap, Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::serialize_with_tag;
use serde_with::skip_serializing_none;
use strum_macros::{AsRefStr, Display, EnumIter};

#[derive(
    Debug, Eq, PartialEq, Clone, Serialize_repr, Deserialize_repr, Display, AsRefStr, EnumIter,
)]
#[repr(u32)]
pub enum NFTokenOfferFlag {
    LsfSellNFToken = 0x00000001,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NFTokenOffer<'a> {
    pub ledger_entry_type: LedgerEntryType,
    #[serde(with = "lgr_obj_flags")]
    pub flags: Vec<NFTokenOfferFlag>,
    /// The object ID of a single object to retrieve from the ledger, as a
    /// 64-character (256-bit) hexadecimal string.
    #[serde(rename = "index")]
    pub index: &'a str,
    pub amount: Amount,
    #[serde(rename = "NFTokenID")]
    pub nftoken_id: &'a str,
    pub owner: &'a str,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: &'a str,
    pub previous_txn_lgr_seq: u32,
    pub destination: Option<&'a str>,
    pub expiration: Option<u32>,
    #[serde(rename = "NFTokenOfferNode")]
    pub nftoken_offer_node: Option<&'a str>,
    pub owner_node: Option<&'a str>,
}

impl<'a> Default for NFTokenOffer<'a> {
    fn default() -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::NFTokenOffer,
            flags: Default::default(),
            index: Default::default(),
            amount: Default::default(),
            nftoken_id: Default::default(),
            owner: Default::default(),
            previous_txn_id: Default::default(),
            previous_txn_lgr_seq: Default::default(),
            destination: Default::default(),
            expiration: Default::default(),
            nftoken_offer_node: Default::default(),
            owner_node: Default::default(),
        }
    }
}

impl<'a> Model for NFTokenOffer<'a> {}

impl<'a> NFTokenOffer<'a> {
    pub fn new(
        flags: Vec<NFTokenOfferFlag>,
        index: &'a str,
        amount: Amount,
        nftoken_id: &'a str,
        owner: &'a str,
        previous_txn_id: &'a str,
        previous_txn_lgr_seq: u32,
        destination: Option<&'a str>,
        expiration: Option<u32>,
        nftoken_offer_node: Option<&'a str>,
        owner_node: Option<&'a str>,
    ) -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::NFTokenOffer,
            flags,
            index,
            amount,
            nftoken_id,
            owner,
            previous_txn_id,
            previous_txn_lgr_seq,
            destination,
            expiration,
            nftoken_offer_node,
            owner_node,
        }
    }
}

#[cfg(test)]
mod test_serde {
    use super::*;
    use alloc::borrow::Cow;
    use alloc::vec;

    #[test]
    fn test_serialization() {
        let nftoken_offer = NFTokenOffer::new(
            vec![NFTokenOfferFlag::LsfSellNFToken],
            "AEBABA4FAC212BF28E0F9A9C3788A47B085557EC5D1429E7A8266FB859C863B3",
            Amount::Xrp(Cow::from("1000000")),
            "00081B5825A08C22787716FA031B432EBBC1B101BB54875F0002D2A400000000",
            "rhRxL3MNvuKEjWjL7TBbZSDacb8PmzAd7m",
            "BFA9BE27383FA315651E26FDE1FA30815C5A5D0544EE10EC33D3E92532993769",
            75443565,
            None,
            None,
            Some("0"),
            Some("17"),
        );
        let nftoken_offer_json = serde_json::to_string(&nftoken_offer).unwrap();
        let actual = nftoken_offer_json.as_str();
        let expected = r#"{"LedgerEntryType":"NFTokenOffer","Flags":1,"index":"AEBABA4FAC212BF28E0F9A9C3788A47B085557EC5D1429E7A8266FB859C863B3","Amount":"1000000","NFTokenID":"00081B5825A08C22787716FA031B432EBBC1B101BB54875F0002D2A400000000","Owner":"rhRxL3MNvuKEjWjL7TBbZSDacb8PmzAd7m","PreviousTxnID":"BFA9BE27383FA315651E26FDE1FA30815C5A5D0544EE10EC33D3E92532993769","PreviousTxnLgrSeq":75443565,"NFTokenOfferNode":"0","OwnerNode":"17"}"#;

        assert_eq!(expected, actual);
    }
}
