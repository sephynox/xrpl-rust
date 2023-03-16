use crate::models::ledger::LedgerEntryType;
use crate::models::{Amount, Currency, Model};
use alloc::borrow::Cow;
use alloc::vec::Vec;
use derive_new::new;
use serde::{ser::SerializeMap, Deserialize, Serialize};

use crate::serialize_with_tag;
use serde_with::skip_serializing_none;

serialize_with_tag! {
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, new, Default)]
pub struct AuthAccount<'a> {
    pub account: Cow<'a, str>,
}
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, new, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AuctionSlot<'a> {
    pub account: Cow<'a, str>,
    pub auth_accounts: Vec<AuthAccount<'a>>,
    pub discounted_fee: u32,
    pub expiration: u32,
    pub price: Amount,
}

serialize_with_tag! {
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, new, Default)]
pub struct VoteEntry<'a> {
    pub account: Cow<'a, str>,
    pub trading_fee: u16,
    pub vote_weight: u32,
}
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AMM<'a> {
    pub ledger_entry_type: LedgerEntryType,
    /// Currently there are no flags for the AMM ledger object
    pub flags: u32,
    /// The object ID of a single object to retrieve from the ledger, as a
    /// 64-character (256-bit) hexadecimal string.
    #[serde(rename = "index")]
    pub index: &'a str,
    #[serde(rename = "AMMAccount")]
    pub amm_account: &'a str,
    pub asset: Currency,
    pub asset2: Currency,
    pub auction_slot: Option<AuctionSlot<'a>>,
    #[serde(rename = "LPTokenBalance")]
    pub lptoken_balance: Amount,
    pub trading_fee: u16,
    pub vote_slots: Option<Vec<VoteEntry<'a>>>,
}

impl<'a> Default for AMM<'a> {
    fn default() -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::AMM,
            flags: Default::default(),
            index: Default::default(),
            amm_account: Default::default(),
            asset: Default::default(),
            asset2: Default::default(),
            auction_slot: Default::default(),
            lptoken_balance: Default::default(),
            trading_fee: Default::default(),
            vote_slots: Default::default(),
        }
    }
}

impl<'a> Model for AMM<'a> {}

impl<'a> AMM<'a> {
    pub fn new(
        index: &'a str,
        amm_account: &'a str,
        asset: Currency,
        asset2: Currency,
        lptoken_balance: Amount,
        trading_fee: u16,
        auction_slot: Option<AuctionSlot<'a>>,
        vote_slots: Option<Vec<VoteEntry<'a>>>,
    ) -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::AMM,
            flags: 0,
            index,
            amm_account,
            asset,
            asset2,
            auction_slot,
            lptoken_balance,
            trading_fee,
            vote_slots,
        }
    }
}

#[cfg(test)]
mod test_serde {
    use crate::models::ledger::amm::{AuctionSlot, AuthAccount, VoteEntry, AMM};
    use crate::models::{Amount, Currency};
    use alloc::borrow::Cow;
    use alloc::vec;

    #[test]
    fn test_serialize() {
        let amm = AMM::new(
            "ForTest",
            "rE54zDvgnghAoPopCgvtiqWNq3dU5y836S",
            Currency::Xrp,
            Currency::IssuedCurrency {
                currency: Cow::from("TST"),
                issuer: Cow::from("rP9jPyP5kyvFRb6ZiRghAGw5u8SGAmU4bd"),
            },
            Amount::IssuedCurrency {
                currency: Cow::from("039C99CD9AB0B70B32ECDA51EAAE471625608EA2"),
                issuer: Cow::from("rE54zDvgnghAoPopCgvtiqWNq3dU5y836S"),
                value: Cow::from("71150.53584131501"),
            },
            600,
            Some(AuctionSlot::new(
                Cow::from("rJVUeRqDFNs2xqA7ncVE6ZoAhPUoaJJSQm"),
                vec![
                    AuthAccount::new(Cow::from("rMKXGCbJ5d8LbrqthdG46q3f969MVK2Qeg")),
                    AuthAccount::new(Cow::from("rBepJuTLFJt3WmtLXYAxSjtBWAeQxVbncv")),
                ],
                0,
                721870180,
                Amount::IssuedCurrency {
                    currency: Cow::from("039C99CD9AB0B70B32ECDA51EAAE471625608EA2"),
                    issuer: Cow::from("rE54zDvgnghAoPopCgvtiqWNq3dU5y836S"),
                    value: Cow::from("0.8696263565463045"),
                },
            )),
            Some(vec![VoteEntry::new(
                Cow::from("rJVUeRqDFNs2xqA7ncVE6ZoAhPUoaJJSQm"),
                600,
                100000,
            )]),
        );
        let amm_json = serde_json::to_string(&amm).unwrap();
        let actual = amm_json.as_str();
        let expected = r#"{"LedgerEntryType":"AMM","Flags":0,"index":"ForTest","AMMAccount":"rE54zDvgnghAoPopCgvtiqWNq3dU5y836S","Asset":{"currency":"XRP"},"Asset2":{"currency":"TST","issuer":"rP9jPyP5kyvFRb6ZiRghAGw5u8SGAmU4bd"},"AuctionSlot":{"Account":"rJVUeRqDFNs2xqA7ncVE6ZoAhPUoaJJSQm","AuthAccounts":[{"AuthAccount":{"Account":"rMKXGCbJ5d8LbrqthdG46q3f969MVK2Qeg"}},{"AuthAccount":{"Account":"rBepJuTLFJt3WmtLXYAxSjtBWAeQxVbncv"}}],"DiscountedFee":0,"Expiration":721870180,"Price":{"currency":"039C99CD9AB0B70B32ECDA51EAAE471625608EA2","issuer":"rE54zDvgnghAoPopCgvtiqWNq3dU5y836S","value":"0.8696263565463045"}},"LPTokenBalance":{"currency":"039C99CD9AB0B70B32ECDA51EAAE471625608EA2","issuer":"rE54zDvgnghAoPopCgvtiqWNq3dU5y836S","value":"71150.53584131501"},"TradingFee":600,"VoteSlots":[{"VoteEntry":{"Account":"rJVUeRqDFNs2xqA7ncVE6ZoAhPUoaJJSQm","TradingFee":600,"VoteWeight":100000}}]}"#;

        assert_eq!(expected, actual);
    }

    // TODO: test_deserialize
}
