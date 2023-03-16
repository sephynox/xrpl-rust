use crate::_serde::lgr_obj_flags;
use crate::models::ledger::LedgerEntryType;
use crate::models::{Amount, Model};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum_macros::{AsRefStr, Display, EnumIter};

use serde_with::skip_serializing_none;

#[derive(
    Debug, Eq, PartialEq, Clone, Serialize_repr, Deserialize_repr, Display, AsRefStr, EnumIter,
)]
#[repr(u32)]
pub enum RippleStateFlag {
    LsfLowReserve = 0x00010000,
    LsfHighReserve = 0x00020000,
    LsfLowAuth = 0x00040000,
    LsfHighAuth = 0x00080000,
    LsfLowNoRipple = 0x00100000,
    LsfHighNoRipple = 0x00200000,
    LsfLowFreeze = 0x00400000,
    LsfHighFreeze = 0x00800000,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct RippleState<'a> {
    ledger_entry_type: LedgerEntryType,
    #[serde(with = "lgr_obj_flags")]
    flags: Vec<RippleStateFlag>,
    /// The object ID of a single object to retrieve from the ledger, as a
    /// 64-character (256-bit) hexadecimal string.
    #[serde(rename = "index")]
    pub index: &'a str,
    pub balance: Amount,
    pub high_limit: Amount,
    pub high_node: &'a str,
    pub low_limit: Amount,
    pub low_node: &'a str,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: &'a str,
    pub previous_txn_lgr_seq: u32,
    pub high_quality_in: Option<u32>,
    pub high_quality_out: Option<u32>,
    pub low_quality_in: Option<u32>,
    pub low_quality_out: Option<u32>,
}

impl<'a> Default for RippleState<'a> {
    fn default() -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::RippleState,
            flags: Default::default(),
            index: Default::default(),
            balance: Default::default(),
            high_limit: Default::default(),
            high_node: Default::default(),
            low_limit: Default::default(),
            low_node: Default::default(),
            previous_txn_id: Default::default(),
            previous_txn_lgr_seq: Default::default(),
            high_quality_in: Default::default(),
            high_quality_out: Default::default(),
            low_quality_in: Default::default(),
            low_quality_out: Default::default(),
        }
    }
}

impl<'a> Model for RippleState<'a> {}

impl<'a> RippleState<'a> {
    pub fn new(
        flags: Vec<RippleStateFlag>,
        index: &'a str,
        balance: Amount,
        high_limit: Amount,
        high_node: &'a str,
        low_limit: Amount,
        low_node: &'a str,
        previous_txn_id: &'a str,
        previous_txn_lgr_seq: u32,
        high_quality_in: Option<u32>,
        high_quality_out: Option<u32>,
        low_quality_in: Option<u32>,
        low_quality_out: Option<u32>,
    ) -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::RippleState,
            flags,
            index,
            balance,
            high_limit,
            high_node,
            low_limit,
            low_node,
            previous_txn_id,
            previous_txn_lgr_seq,
            high_quality_in,
            high_quality_out,
            low_quality_in,
            low_quality_out,
        }
    }
}

#[cfg(test)]
mod test_serde {
    use super::*;
    use alloc::{borrow::Cow, vec};

    #[test]
    fn test_serialize() {
        let ripple_state = RippleState::new(
            vec![RippleStateFlag::LsfHighReserve, RippleStateFlag::LsfLowAuth],
            "9CA88CDEDFF9252B3DE183CE35B038F57282BC9503CDFA1923EF9A95DF0D6F7B",
            Amount::IssuedCurrency {
                currency: Cow::from("USD"),
                issuer: Cow::from("rrrrrrrrrrrrrrrrrrrrBZbvji"),
                value: Cow::from("-10"),
            },
            Amount::IssuedCurrency {
                currency: Cow::from("USD"),
                issuer: Cow::from("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn"),
                value: Cow::from("110"),
            },
            "0000000000000000",
            Amount::IssuedCurrency {
                currency: Cow::from("USD"),
                issuer: Cow::from("rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW"),
                value: Cow::from("0"),
            },
            "0000000000000000",
            "E3FE6EA3D48F0C2B639448020EA4F03D4F4F8FFDB243A852A0F59177921B4879",
            14090896,
            None,
            None,
            None,
            None,
        );
        let ripple_state_json = serde_json::to_string(&ripple_state).unwrap();
        let actual = ripple_state_json.as_str();
        let expected = r#"{"LedgerEntryType":"RippleState","Flags":393216,"index":"9CA88CDEDFF9252B3DE183CE35B038F57282BC9503CDFA1923EF9A95DF0D6F7B","Balance":{"currency":"USD","issuer":"rrrrrrrrrrrrrrrrrrrrBZbvji","value":"-10"},"HighLimit":{"currency":"USD","issuer":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","value":"110"},"HighNode":"0000000000000000","LowLimit":{"currency":"USD","issuer":"rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW","value":"0"},"LowNode":"0000000000000000","PreviousTxnID":"E3FE6EA3D48F0C2B639448020EA4F03D4F4F8FFDB243A852A0F59177921B4879","PreviousTxnLgrSeq":14090896}"#;

        assert_eq!(expected, actual);
    }
}
