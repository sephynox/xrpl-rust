use crate::Err;
use alloc::borrow::Cow;
use alloc::vec::Vec;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::amount::XRPAmount;
use crate::models::transactions::{CommonFields, XRPLDepositPreauthException};
use crate::models::NoFlags;
use crate::models::{
    model::Model,
    transactions::{Memo, Signer, Transaction, TransactionType},
};

/// A DepositPreauth transaction gives another account pre-approval
/// to deliver payments to the sender of this transaction.
///
/// See DepositPreauth:
/// `<https://xrpl.org/depositpreauth.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DepositPreauth<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(flatten)]
    pub common_fields: CommonFields<'a, NoFlags>,
    /// The custom fields for the DepositPreauth model.
    ///
    /// See DepositPreauth fields:
    /// `<https://xrpl.org/depositpreauth.html#depositpreauth-fields>`
    pub authorize: Option<Cow<'a, str>>,
    pub unauthorize: Option<Cow<'a, str>>,
}

impl<'a: 'static> Model for DepositPreauth<'a> {
    fn get_errors(&self) -> Result<()> {
        match self._get_authorize_and_unauthorize_error() {
            Ok(_no_error) => Ok(()),
            Err(error) => Err!(error),
        }
    }
}

impl<'a> Transaction<NoFlags> for DepositPreauth<'a> {
    fn get_transaction_type(&self) -> TransactionType {
        self.common_fields.get_transaction_type()
    }
}

impl<'a> DepositPreauthError for DepositPreauth<'a> {
    fn _get_authorize_and_unauthorize_error(&self) -> Result<(), XRPLDepositPreauthException> {
        if (self.authorize.is_none() && self.unauthorize.is_none())
            || (self.authorize.is_some() && self.unauthorize.is_some())
        {
            Err(XRPLDepositPreauthException::DefineExactlyOneOf {
                field1: "authorize".into(),
                field2: "unauthorize".into(),
                resource: "".into(),
            })
        } else {
            Ok(())
        }
    }
}

impl<'a> DepositPreauth<'a> {
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
        authorize: Option<Cow<'a, str>>,
        unauthorize: Option<Cow<'a, str>>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                account,
                transaction_type: TransactionType::DepositPreauth,
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
            authorize,
            unauthorize,
        }
    }
}

pub trait DepositPreauthError {
    fn _get_authorize_and_unauthorize_error(&self) -> Result<(), XRPLDepositPreauthException>;
}

#[cfg(test)]
mod test_deposit_preauth_exception {

    use crate::models::Model;
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_authorize_and_unauthorize_error() {
        let deposit_preauth = DepositPreauth::new(
            "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb".into(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        assert_eq!(
            deposit_preauth.validate().unwrap_err().to_string().as_str(),
            "The field `authorize` can not be defined with `unauthorize`. Define exactly one of them. For more information see: "
        );
    }
}

#[cfg(test)]
mod test_serde {
    use super::*;

    #[test]
    fn test_serialize() {
        let default_txn = DepositPreauth::new(
            "rsUiUMpnrgxQp24dJYZDhmV4bE3aBtQyt8".into(),
            None,
            Some("10".into()),
            None,
            None,
            Some(2),
            None,
            None,
            None,
            Some("rEhxGqkqPPSxQ3P25J66ft5TwpzV14k2de".into()),
            None,
        );
        let default_json = r#"{"TransactionType":"DepositPreauth","Account":"rsUiUMpnrgxQp24dJYZDhmV4bE3aBtQyt8","Fee":"10","Sequence":2,"Authorize":"rEhxGqkqPPSxQ3P25J66ft5TwpzV14k2de"}"#;

        let txn_as_string = serde_json::to_string(&default_txn).unwrap();
        let txn_json = txn_as_string.as_str();

        assert_eq!(txn_json, default_json);
    }

    #[test]
    fn test_deserialize() {
        let default_txn = DepositPreauth::new(
            "rsUiUMpnrgxQp24dJYZDhmV4bE3aBtQyt8".into(),
            None,
            Some("10".into()),
            None,
            None,
            Some(2),
            None,
            None,
            None,
            Some("rEhxGqkqPPSxQ3P25J66ft5TwpzV14k2de".into()),
            None,
        );
        let default_json = r#"{"TransactionType":"DepositPreauth","Account":"rsUiUMpnrgxQp24dJYZDhmV4bE3aBtQyt8","Authorize":"rEhxGqkqPPSxQ3P25J66ft5TwpzV14k2de","Fee":"10","Sequence":2}"#;

        let txn_as_obj: DepositPreauth = serde_json::from_str(default_json).unwrap();

        assert_eq!(txn_as_obj, default_txn);
    }
}
