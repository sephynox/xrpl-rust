use alloc::borrow::Cow;
use alloc::vec::Vec;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::NoFlags;
use crate::models::{
    amount::XRPAmount,
    model::Model,
    transactions::{Memo, Signer, Transaction, TransactionType},
};

use super::CommonFields;

/// Add additional XRP to an open payment channel,
/// and optionally update the expiration time of the channel.
///
/// See PaymentChannelFund:
/// `<https://xrpl.org/paymentchannelfund.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentChannelFund<'a> {
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(flatten)]
    pub common_fields: CommonFields<'a, NoFlags>,
    /// The custom fields for the PaymentChannelFund model.
    ///
    /// See PaymentChannelFund fields:
    /// `<https://xrpl.org/paymentchannelfund.html#paymentchannelfund-fields>`
    pub amount: XRPAmount<'a>,
    pub channel: Cow<'a, str>,
    pub expiration: Option<u32>,
}

impl<'a> Model for PaymentChannelFund<'a> {}

impl<'a> Transaction<NoFlags> for PaymentChannelFund<'a> {
    fn get_transaction_type(&self) -> TransactionType {
        self.common_fields.transaction_type.clone()
    }
}

impl<'a> PaymentChannelFund<'a> {
    pub fn new(
        account: Cow<'a, str>,
        account_txn_id: Option<Cow<'a, str>>,
        fee: Option<XRPAmount<'a>>,
        last_ledger_sequence: Option<u32>,
        memos: Option<Vec<Memo>>,
        sequence: Option<u32>,
        signers: Option<Vec<Signer<'a>>>,
        source_tag: Option<u32>,
        ticket_sequence: Option<u32>,
        amount: XRPAmount<'a>,
        channel: Cow<'a, str>,
        expiration: Option<u32>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                account,
                transaction_type: TransactionType::PaymentChannelFund,
                account_txn_id,
                fee,
                flags: None,
                last_ledger_sequence,
                memos,
                sequence,
                signers,
                source_tag,
                ticket_sequence,
            },
            amount,
            channel,
            expiration,
        }
    }
}

#[cfg(test)]
mod test_serde {
    use crate::models::amount::XRPAmount;

    use super::*;

    #[test]
    fn test_serialize() {
        let default_txn = PaymentChannelFund::new(
            "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn".into(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            XRPAmount::from("200000"),
            "C1AE6DDDEEC05CF2978C0BAD6FE302948E9533691DC749DCDD3B9E5992CA6198".into(),
            Some(543171558),
        );
        let default_json = r#"{"TransactionType":"PaymentChannelFund","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Amount":"200000","Channel":"C1AE6DDDEEC05CF2978C0BAD6FE302948E9533691DC749DCDD3B9E5992CA6198","Expiration":543171558}"#;

        let txn_as_string = serde_json::to_string(&default_txn).unwrap();
        let txn_json = txn_as_string.as_str();

        assert_eq!(txn_json, default_json);
    }

    #[test]
    fn test_deserialize() {
        let default_txn = PaymentChannelFund::new(
            "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn".into(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            XRPAmount::from("200000"),
            "C1AE6DDDEEC05CF2978C0BAD6FE302948E9533691DC749DCDD3B9E5992CA6198".into(),
            Some(543171558),
        );
        let default_json = r#"{"Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","TransactionType":"PaymentChannelFund","Channel":"C1AE6DDDEEC05CF2978C0BAD6FE302948E9533691DC749DCDD3B9E5992CA6198","Amount":"200000","Expiration":543171558}"#;

        let txn_as_obj: PaymentChannelFund = serde_json::from_str(default_json).unwrap();

        assert_eq!(txn_as_obj, default_txn);
    }
}
