use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{
    exceptions::{CheckCashException, XRPLModelException, XRPLTransactionException},
    model::Model,
    Amount, CheckCashError, Memo, Signer, Transaction, TransactionType,
};

/// Cancels an unredeemed Check, removing it from the ledger without
/// sending any money. The source or the destination of the check can
/// cancel a Check at any time using this transaction type. If the Check
/// has expired, any address can cancel it.
///
/// See CheckCash:
/// `<https://xrpl.org/checkcash.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CheckCash<'a> {
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::check_cash")]
    pub transaction_type: TransactionType,
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
    pub flags: Option<u32>,
    /// Additional arbitrary information used to identify this transaction.
    pub memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    pub signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the CheckCash model.
    ///
    /// See CheckCash fields:
    /// `<https://xrpl.org/checkcash.html#checkcash-fields>`
    pub check_id: &'a str,
    pub amount: Option<Amount>,
    pub deliver_min: Option<Amount>,
}

impl<'a> Default for CheckCash<'a> {
    fn default() -> Self {
        Self {
            transaction_type: TransactionType::CheckCash,
            account: Default::default(),
            fee: Default::default(),
            sequence: Default::default(),
            last_ledger_sequence: Default::default(),
            account_txn_id: Default::default(),
            signing_pub_key: Default::default(),
            source_tag: Default::default(),
            ticket_sequence: Default::default(),
            txn_signature: Default::default(),
            flags: Default::default(),
            memos: Default::default(),
            signers: Default::default(),
            check_id: Default::default(),
            amount: Default::default(),
            deliver_min: Default::default(),
        }
    }
}

impl<'a> Model for CheckCash<'a> {
    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self._get_amount_and_deliver_min_error() {
            Ok(_no_error) => Ok(()),
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::CheckCashError(error),
            )),
        }
    }
}

impl<'a> Transaction for CheckCash<'a> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl<'a> CheckCashError for CheckCash<'a> {
    fn _get_amount_and_deliver_min_error(&self) -> Result<(), CheckCashException> {
        match self.amount.is_none() && self.deliver_min.is_none() {
            true => Err(CheckCashException::InvalidMustSetAmountOrDeliverMin),
            false => match self.amount.is_some() && self.deliver_min.is_some() {
                true => Err(CheckCashException::InvalidMustNotSetAmountAndDeliverMin),
                false => Ok(()),
            },
        }
    }
}

impl<'a> CheckCash<'a> {
    fn new(
        account: &'a str,
        check_id: &'a str,
        fee: Option<&'a str>,
        sequence: Option<u32>,
        last_ledger_sequence: Option<u32>,
        account_txn_id: Option<&'a str>,
        signing_pub_key: Option<&'a str>,
        source_tag: Option<u32>,
        ticket_sequence: Option<u32>,
        txn_signature: Option<&'a str>,
        memos: Option<Vec<Memo<'a>>>,
        signers: Option<Vec<Signer<'a>>>,
        amount: Option<Amount>,
        deliver_min: Option<Amount>,
    ) -> Self {
        Self {
            transaction_type: TransactionType::CheckCash,
            account,
            fee,
            sequence,
            last_ledger_sequence,
            account_txn_id,
            signing_pub_key,
            source_tag,
            ticket_sequence,
            txn_signature,
            flags: None,
            memos,
            signers,
            check_id,
            amount,
            deliver_min,
        }
    }
}

#[cfg(test)]
mod test_check_cash_error {
    use crate::models::{
        exceptions::{CheckCashException, XRPLModelException, XRPLTransactionException},
        Amount, Model, TransactionType,
    };

    use alloc::borrow::Cow;

    use super::CheckCash;

    #[test]
    fn test_amount_and_deliver_min_error() {
        let mut check_cash = CheckCash {
            transaction_type: TransactionType::CheckCash,
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
            check_id: "",
            amount: None,
            deliver_min: None,
        };
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::CheckCashError(
                CheckCashException::InvalidMustSetAmountOrDeliverMin,
            ));
        assert_eq!(check_cash.validate(), Err(expected_error));

        check_cash.amount = Some(Amount::Xrp(Cow::Borrowed("1000000")));
        check_cash.deliver_min = Some(Amount::Xrp(Cow::Borrowed("100000")));
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::CheckCashError(
                CheckCashException::InvalidMustNotSetAmountAndDeliverMin,
            ));
        assert_eq!(check_cash.validate(), Err(expected_error));
    }
}
