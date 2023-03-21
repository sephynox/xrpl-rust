use crate::Err;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use alloc::string::ToString;

use crate::models::transactions::XRPLUNLModifyException;
use crate::models::{
    amount::XRPAmount, model::Model, Transaction, TransactionType, UNLModifyError,
};

/// See UNLModify:
/// `<https://xrpl.org/unlmodify.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct UNLModify<'a> {
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::unl_modify")]
    pub transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    pub account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    pub fee: Option<XRPAmount<'a>>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    pub sequence: Option<u32>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    pub signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    pub source_tag: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    pub txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    pub flags: Option<u32>,
    /// The custom fields for the UNLModify model.
    ///
    /// See UNLModify fields:
    /// `<https://xrpl.org/unlmodify.html#unlmodify-fields>`
    pub ledger_sequence: u32,
    pub unlmodify_disabling: u8,
    pub unlmodify_validator: &'a str,
}

impl<'a: 'static> Model for UNLModify<'a> {
    fn get_errors(&self) -> Result<()> {
        match self._get_unl_modify_error() {
            Err(error) => Err!(error),
            Ok(_no_error) => Ok(()),
        }
    }
}

impl<'a> Transaction for UNLModify<'a> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

// TODO: Enum for unlmodify_disabling to make looking for error obsolete
impl<'a> UNLModifyError for UNLModify<'a> {
    fn _get_unl_modify_error(&self) -> Result<(), XRPLUNLModifyException> {
        let possible_unlmodify_disabling: [u8; 2] = [0, 1];
        if !possible_unlmodify_disabling.contains(&self.unlmodify_disabling) {
            Err(XRPLUNLModifyException::InvalidValue {
                field: "unlmodify_disabling",
                expected: "0 or 1",
                found: self.unlmodify_disabling as u32,
                resource: "",
            })
        } else {
            Ok(())
        }
    }
}

impl<'a> UNLModify<'a> {
    fn new(
        account: &'a str,
        ledger_sequence: u32,
        unlmodify_disabling: u8,
        unlmodify_validator: &'a str,
        fee: Option<XRPAmount<'a>>,
        sequence: Option<u32>,
        signing_pub_key: Option<&'a str>,
        source_tag: Option<u32>,
        txn_signature: Option<&'a str>,
    ) -> Self {
        Self {
            transaction_type: TransactionType::UNLModify,
            account,
            fee,
            sequence,
            signing_pub_key,
            source_tag,
            txn_signature,
            flags: None,
            ledger_sequence,
            unlmodify_disabling,
            unlmodify_validator,
        }
    }
}

#[cfg(test)]
mod test_unl_modify_error {

    use crate::models::{Model, TransactionType};
    use alloc::string::ToString;

    use super::UNLModify;

    #[test]
    fn test_unlmodify_disabling_error() {
        let unl_modify = UNLModify {
            transaction_type: TransactionType::UNLModify,
            account: "",
            fee: None,
            sequence: None,
            signing_pub_key: None,
            source_tag: None,
            txn_signature: None,
            flags: None,
            ledger_sequence: 1600000,
            unlmodify_disabling: 3,
            unlmodify_validator:
                "ED6629D456285AE3613B285F65BBFF168D695BA3921F309949AFCD2CA7AFEC16FE",
        };

        assert_eq!(
            unl_modify.validate().unwrap_err().to_string().as_str(),
            "The field `unlmodify_disabling` has an invalid value (expected 0 or 1, found 3). For more information see: "
        );
    }
}
