use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{default_zero, model::Model, Memo, Signer, Transaction, TransactionType};

/// An AccountDelete transaction deletes an account and any objects it
/// owns in the XRP Ledger, if possible, sending the account's remaining
/// XRP to a specified destination account. See Deletion of Accounts for
/// the requirements to delete an account.
///
/// See AccountDelete:
/// `<https://xrpl.org/accountdelete.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct AccountDelete<'a> {
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::account_set")]
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
    // The custom fields for the AccountDelete model.
    //
    // See AccountDelete fields:
    // `<https://xrpl.org/accountdelete.html#accountdelete-fields>`
    /// The address of an account to receive any leftover XRP after
    /// deleting the sending account. Must be a funded account in
    /// the ledger, and must not be the sending account.
    pub destination: &'a str,
    /// Arbitrary destination tag that identifies a hosted
    /// recipient or other information for the recipient
    /// of the deleted account's leftover XRP.
    pub destination_tag: Option<u32>,
}

impl Model for AccountDelete<'static> {}

impl Transaction for AccountDelete<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}
