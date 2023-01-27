use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{
    default_zero,
    exceptions::{EscrowCreateException, XRPLModelException, XRPLTransactionException},
    model::Model,
    CurrencyAmount, EscrowCreateError, Memo, Signer, Transaction, TransactionType,
};

/// Creates an Escrow, which sequests XRP until the escrow process either finishes or is canceled.
///
/// See EscrowCreate:
/// `<https://xrpl.org/escrowcreate.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct EscrowCreate<'a> {
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::escrow_create")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    pub account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    pub fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    pub sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    pub last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    pub account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    pub signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    pub source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    pub ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    pub txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    #[serde(default = "default_zero")]
    flags: Option<u32>,
    /// Additional arbitrary information used to identify this transaction.
    pub memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    pub signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the EscrowCreate model.
    ///
    /// See EscrowCreate fields:
    /// `<https://xrpl.org/escrowcreate.html#escrowcreate-flags>`
    pub amount: CurrencyAmount,
    pub destination: &'a str,
    pub destination_tag: Option<&'a str>,
    pub cancel_after: Option<u32>,
    pub finish_after: Option<u32>,
    pub condition: Option<&'a str>,
}

impl Model for EscrowCreate<'static> {
    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self._get_finish_after_error() {
            Ok(_no_error) => Ok(()),
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::EscrowCreateError(error),
            )),
        }
    }
}

impl Transaction for EscrowCreate<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl EscrowCreateError for EscrowCreate<'static> {
    fn _get_finish_after_error(&self) -> Result<(), EscrowCreateException> {
        match self.finish_after {
            Some(finish_after) => match self.cancel_after {
                Some(cancel_after) => match finish_after >= cancel_after {
                    true => {
                        Err(EscrowCreateException::InvalidCancelAfterMustNotBeBeforeFinishAfter)
                    }
                    false => Ok(()),
                },
                None => Ok(()),
            },
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod test_escrow_create_errors {
    use crate::models::{
        exceptions::{EscrowCreateException, XRPLModelException, XRPLTransactionException},
        CurrencyAmount, Model, TransactionType,
    };

    use alloc::borrow::Cow;

    use super::EscrowCreate;

    #[test]
    fn test_cancel_after_error() {
        let escrow_create = EscrowCreate {
            transaction_type: TransactionType::EscrowCreate,
            account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            fee: None,
            sequence: None,
            last_ledger_sequence: None,
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: None,
            memos: None,
            signers: None,
            amount: CurrencyAmount::Xrp(Cow::Borrowed("100000000")),
            destination: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            destination_tag: None,
            cancel_after: Some(13298498),
            finish_after: Some(14359039),
            condition: None,
        };
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::EscrowCreateError(
                EscrowCreateException::InvalidCancelAfterMustNotBeBeforeFinishAfter,
            ));
        assert_eq!(escrow_create.validate(), Err(expected_error));
    }
}
