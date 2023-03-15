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
    account: Cow<'a, str>,
}
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, new, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AuctionSlot<'a> {
    account: Cow<'a, str>,
    auth_accounts: Vec<AuthAccount<'a>>,
    discounted_fee: u32,
    expiration: u32,
    price: Amount,
}

serialize_with_tag! {
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, new, Default)]
pub struct VoteEntry<'a> {
    account: Cow<'a, str>,
    trading_fee: u32,
    vote_weight: u32,
}
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AMM<'a> {
    ledger_entry_type: LedgerEntryType,
    #[serde(rename = "AMMAccount")]
    amm_account: &'a str,
    asset: Currency,
    asset2: Currency,
    auction_slot: Option<AuctionSlot<'a>>,
    /// Currently there are no flags for the AMM ledger object
    flags: u32,
    #[serde(rename = "index")]
    index: &'a str,
    #[serde(rename = "LPTokenBalance")]
    lptoken_balance: Amount,
    trading_fee: u16,
    vote_slots: Option<Vec<VoteEntry<'a>>>,
}

impl<'a> Default for AMM<'a> {
    fn default() -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::AMM,
            amm_account: Default::default(),
            asset: Default::default(),
            asset2: Default::default(),
            auction_slot: Default::default(),
            flags: Default::default(),
            index: Default::default(),
            lptoken_balance: Default::default(),
            trading_fee: Default::default(),
            vote_slots: Default::default(),
        }
    }
}

impl<'a> Model for AMM<'a> {}

impl<'a> AMM<'a> {
    pub fn new(
        amm_account: &'a str,
        asset: Currency,
        asset2: Currency,
        flags: u32,
        index: &'a str,
        lptoken_balance: Amount,
        trading_fee: u16,
        auction_slot: Option<AuctionSlot<'a>>,
        vote_slots: Option<Vec<VoteEntry<'a>>>,
    ) -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::AMM,
            amm_account,
            asset,
            asset2,
            auction_slot,
            flags,
            index,
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
            "rE54zDvgnghAoPopCgvtiqWNq3dU5y836S",
            Currency::Xrp,
            Currency::IssuedCurrency {
                currency: Cow::from("TST"),
                issuer: Cow::from("rP9jPyP5kyvFRb6ZiRghAGw5u8SGAmU4bd"),
            },
            0,
            "ForTest",
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
        let expected = r#"{"LedgerEntryType":"AMM","AMMAccount":"rE54zDvgnghAoPopCgvtiqWNq3dU5y836S","Asset":{"currency":"XRP"},"Asset2":{"currency":"TST","issuer":"rP9jPyP5kyvFRb6ZiRghAGw5u8SGAmU4bd"},"AuctionSlot":{"Account":"rJVUeRqDFNs2xqA7ncVE6ZoAhPUoaJJSQm","AuthAccounts":[{"AuthAccount":{"Account":"rMKXGCbJ5d8LbrqthdG46q3f969MVK2Qeg"}},{"AuthAccount":{"Account":"rBepJuTLFJt3WmtLXYAxSjtBWAeQxVbncv"}}],"DiscountedFee":0,"Expiration":721870180,"Price":{"currency":"039C99CD9AB0B70B32ECDA51EAAE471625608EA2","issuer":"rE54zDvgnghAoPopCgvtiqWNq3dU5y836S","value":"0.8696263565463045"}},"Flags":0,"index":"ForTest","LPTokenBalance":{"currency":"039C99CD9AB0B70B32ECDA51EAAE471625608EA2","issuer":"rE54zDvgnghAoPopCgvtiqWNq3dU5y836S","value":"71150.53584131501"},"TradingFee":600,"VoteSlots":[{"VoteEntry":{"Account":"rJVUeRqDFNs2xqA7ncVE6ZoAhPUoaJJSQm","TradingFee":600,"VoteWeight":100000}}]}"#;

        assert_eq!(expected, actual);
    }

    // TODO: test_deserialize
}
