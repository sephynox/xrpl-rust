use alloc::borrow::Cow;
use alloc::vec::Vec;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::amount::XRPAmount;
use crate::models::NoFlags;
use crate::models::{
    model::Model,
    transactions::{Memo, Signer, Transaction, TransactionType},
};

use super::CommonFields;

/// You can protect your account by assigning a regular key pair to
/// it and using it instead of the master key pair to sign transactions
/// whenever possible. If your regular key pair is compromised, but
/// your master key pair is not, you can use a SetRegularKey transaction
/// to regain control of your account.
///
/// See SetRegularKey:
/// `<https://xrpl.org/setregularkey.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SetRegularKey<'a> {
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
    /// The custom fields for the SetRegularKey model.
    ///
    /// See SetRegularKey fields:
    /// `<https://xrpl.org/setregularkey.html#setregularkey-fields>`
    pub regular_key: Option<Cow<'a, str>>,
}

impl<'a> Model for SetRegularKey<'a> {}

impl<'a> Transaction<NoFlags> for SetRegularKey<'a> {
    fn get_transaction_type(&self) -> TransactionType {
        self.common_fields.transaction_type.clone()
    }
}

impl<'a> SetRegularKey<'a> {
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
        regular_key: Option<Cow<'a, str>>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                account,
                transaction_type: TransactionType::SetRegularKey,
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
            regular_key,
        }
    }
}

#[cfg(test)]
mod test_serde {
    use super::*;

    #[test]
    fn test_serialize() {
        let default_txn = SetRegularKey::new(
            "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn".into(),
            None,
            Some("12".into()),
            None,
            None,
            None,
            None,
            None,
            None,
            Some("rAR8rR8sUkBoCZFawhkWzY4Y5YoyuznwD".into()),
        );
        let default_json = r#"{"TransactionType":"SetRegularKey","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","RegularKey":"rAR8rR8sUkBoCZFawhkWzY4Y5YoyuznwD"}"#;

        let txn_as_string = serde_json::to_string(&default_txn).unwrap();
        let txn_json = txn_as_string.as_str();

        assert_eq!(txn_json, default_json);
    }

    #[test]
    fn test_deserialize() {
        let default_txn = SetRegularKey::new(
            "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn".into(),
            None,
            Some("12".into()),
            None,
            None,
            None,
            None,
            None,
            None,
            Some("rAR8rR8sUkBoCZFawhkWzY4Y5YoyuznwD".into()),
        );
        let default_json = r#"{"TransactionType":"SetRegularKey","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","RegularKey":"rAR8rR8sUkBoCZFawhkWzY4Y5YoyuznwD"}"#;

        let txn_as_obj: SetRegularKey = serde_json::from_str(default_json).unwrap();

        assert_eq!(txn_as_obj, default_txn);
    }
}
