use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{
    default_zero,
    exceptions::{EscrowFinishException, XRPLModelException, XRPLTransactionException},
    model::Model,
    EscrowFinishError, Memo, Signer, Transaction, TransactionType,
};

/// Finishes an Escrow and delivers XRP from a held payment to the recipient.
///
/// See EscrowFinish:
/// `<https://xrpl.org/escrowfinish.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct EscrowFinish<'a> {
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::escrow_finish")]
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
    /// The custom fields for the EscrowFinish model.
    ///
    /// See EscrowFinish fields:
    /// `<https://xrpl.org/escrowfinish.html#escrowfinish-fields>`
    pub owner: &'a str,
    pub offer_sequence: u32,
    pub condition: Option<&'a str>,
    pub fulfillment: Option<&'a str>,
}

impl Model for EscrowFinish<'static> {
    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self._get_condition_and_fulfillment_error() {
            Ok(_no_error) => Ok(()),
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::EscrowFinishError(error),
            )),
        }
    }
}

impl From<&EscrowFinish<'static>> for u32 {
    fn from(_: &EscrowFinish<'static>) -> Self {
        0
    }
}

impl Transaction for EscrowFinish<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl EscrowFinishError for EscrowFinish<'static> {
    fn _get_condition_and_fulfillment_error(&self) -> Result<(), EscrowFinishException> {
        match (self.condition.is_some() && self.fulfillment.is_none())
            || (self.condition.is_none() && self.condition.is_some())
        {
            true => Err(EscrowFinishException::InvalidIfOneSetBothConditionAndFulfillmentMustBeSet),
            false => Ok(()),
        }
    }
}

#[cfg(test)]
mod test_escrow_finish_errors {
    use crate::models::{
        exceptions::{EscrowFinishException, XRPLModelException, XRPLTransactionException},
        Model, TransactionType,
    };

    use super::EscrowFinish;

    #[test]
    fn test_condition_and_fulfillment_error() {
        let escrow_finish = EscrowFinish {
            transaction_type: TransactionType::EscrowCancel,
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
            owner: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            offer_sequence: 10,
            condition: Some(
                "A0258020E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855810100",
            ),
            fulfillment: None,
        };
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::EscrowFinishError(
                EscrowFinishException::InvalidIfOneSetBothConditionAndFulfillmentMustBeSet,
            ));
        assert_eq!(escrow_finish.validate(), Err(expected_error));
    }
}
