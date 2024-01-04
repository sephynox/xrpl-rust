use alloc::borrow::Cow;
use alloc::vec::Vec;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::amount::XRPAmount;
use crate::models::transactions::{CommonFields, NoFlags};
use crate::models::{
    amount::Amount,
    model::Model,
    transactions::{Transaction, TransactionType},
};

use super::{Memo, Signer};

/// Create a Check object in the ledger, which is a deferred
/// payment that can be cashed by its intended destination.
///
/// See CheckCreate:
/// `<https://xrpl.org/checkcreate.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CheckCreate<'a> {
    // The base fields for all transaction models.
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(flatten)]
    pub common_fields: CommonFields<'a, NoFlags>,
    /// The custom fields for the CheckCreate model.
    ///
    /// See CheckCreate fields:
    /// `<https://xrpl.org/checkcreate.html#checkcreate-fields>`
    pub destination: Cow<'a, str>,
    pub send_max: Amount<'a>,
    pub destination_tag: Option<u32>,
    pub expiration: Option<u32>,
    #[serde(rename = "InvoiceID")]
    pub invoice_id: Option<Cow<'a, str>>,
}

impl<'a> Model for CheckCreate<'a> {}

impl<'a> Transaction<NoFlags> for CheckCreate<'a> {
    fn get_transaction_type(&self) -> TransactionType {
        self.common_fields.get_transaction_type()
    }
}

impl<'a> CheckCreate<'a> {
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
        destination: Cow<'a, str>,
        send_max: Amount<'a>,
        destination_tag: Option<u32>,
        expiration: Option<u32>,
        invoice_id: Option<Cow<'a, str>>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                account,
                transaction_type: TransactionType::CheckCreate,
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
            destination,
            send_max,
            destination_tag,
            expiration,
            invoice_id,
        }
    }
}

#[cfg(test)]
mod test_serde {
    use super::*;

    #[test]
    fn test_serialize() {
        let default_txn = CheckCreate::new(
            "rUn84CUYbNjRoTQ6mSW7BVJPSVJNLb1QLo".into(),
            None,
            Some("12".into()),
            None,
            None,
            None,
            None,
            None,
            None,
            "rfkE1aSy9G8Upk4JssnwBxhEv5p4mn2KTy".into(),
            "100000000".into(),
            Some(1),
            Some(570113521),
            Some("6F1DFD1D0FE8A32E40E1F2C05CF1C15545BAB56B617F9C6C2D63A6B704BEF59B".into()),
        );
        let default_json = r#"{"TransactionType":"CheckCreate","Account":"rUn84CUYbNjRoTQ6mSW7BVJPSVJNLb1QLo","Fee":"12","Destination":"rfkE1aSy9G8Upk4JssnwBxhEv5p4mn2KTy","SendMax":"100000000","DestinationTag":1,"Expiration":570113521,"InvoiceID":"6F1DFD1D0FE8A32E40E1F2C05CF1C15545BAB56B617F9C6C2D63A6B704BEF59B"}"#;

        let txn_as_string = serde_json::to_string(&default_txn).unwrap();
        let txn_json = txn_as_string.as_str();

        assert_eq!(txn_json, default_json);
    }

    #[test]
    fn test_deserialize() {
        let default_txn = CheckCreate::new(
            "rUn84CUYbNjRoTQ6mSW7BVJPSVJNLb1QLo".into(),
            None,
            Some("12".into()),
            None,
            None,
            None,
            None,
            None,
            None,
            "rfkE1aSy9G8Upk4JssnwBxhEv5p4mn2KTy".into(),
            "100000000".into(),
            Some(1),
            Some(570113521),
            Some("6F1DFD1D0FE8A32E40E1F2C05CF1C15545BAB56B617F9C6C2D63A6B704BEF59B".into()),
        );
        let default_json = r#"{"TransactionType":"CheckCreate","Account":"rUn84CUYbNjRoTQ6mSW7BVJPSVJNLb1QLo","Destination":"rfkE1aSy9G8Upk4JssnwBxhEv5p4mn2KTy","SendMax":"100000000","Expiration":570113521,"InvoiceID":"6F1DFD1D0FE8A32E40E1F2C05CF1C15545BAB56B617F9C6C2D63A6B704BEF59B","DestinationTag":1,"Fee":"12"}"#;

        let txn_as_obj: CheckCreate = serde_json::from_str(default_json).unwrap();

        assert_eq!(txn_as_obj, default_txn);
    }
}
