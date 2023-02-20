use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{
    exceptions::{EscrowCreateException, XRPLModelException, XRPLTransactionException},
    model::Model,
    Amount, EscrowCreateError, Memo, Signer, Transaction, TransactionType,
};

/// Creates an Escrow, which sequests XRP until the escrow process either finishes or is canceled.
///
/// See EscrowCreate:
/// `<https://xrpl.org/escrowcreate.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
    /// The custom fields for the EscrowCreate model.
    ///
    /// See EscrowCreate fields:
    /// `<https://xrpl.org/escrowcreate.html#escrowcreate-flags>`
    pub amount: Amount,
    pub destination: &'a str,
    pub destination_tag: Option<u32>,
    pub cancel_after: Option<u32>,
    pub finish_after: Option<u32>,
    pub condition: Option<&'a str>,
}

impl<'a> Default for EscrowCreate<'a> {
    fn default() -> Self {
        Self {
            transaction_type: TransactionType::EscrowCreate,
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
            amount: Default::default(),
            destination: Default::default(),
            destination_tag: Default::default(),
            cancel_after: Default::default(),
            finish_after: Default::default(),
            condition: Default::default(),
        }
    }
}

impl<'a> Model for EscrowCreate<'a> {
    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self._get_finish_after_error() {
            Ok(_no_error) => Ok(()),
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::EscrowCreateError(error),
            )),
        }
    }
}

impl<'a> Transaction for EscrowCreate<'a> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl<'a> EscrowCreateError for EscrowCreate<'a> {
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

impl<'a> EscrowCreate<'a> {
    fn new(
        account: &'a str,
        amount: Amount,
        destination: &'a str,
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
        destination_tag: Option<u32>,
        cancel_after: Option<u32>,
        finish_after: Option<u32>,
        condition: Option<&'a str>,
    ) -> Self {
        Self {
            transaction_type: TransactionType::EscrowCreate,
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
            amount,
            destination,
            destination_tag,
            cancel_after,
            finish_after,
            condition,
        }
    }
}

#[cfg(test)]
mod test_escrow_create_errors {
    use crate::models::{
        exceptions::{EscrowCreateException, XRPLModelException, XRPLTransactionException},
        Amount, Model, TransactionType,
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
            amount: Amount::Xrp(Cow::Borrowed("100000000")),
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

#[cfg(test)]
mod test_serde {
    use super::*;
    use alloc::borrow::Cow::Borrowed;

    #[test]
    fn test_serialize() {
        let default_txn = EscrowCreate::new(
            "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
            Amount::Xrp(Borrowed("10000")),
            "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW",
            None,
            None,
            None,
            None,
            None,
            Some(11747),
            None,
            None,
            None,
            None,
            Some(23480),
            Some(533257958),
            Some(533171558),
            Some("A0258020E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855810100"),
        );
        let default_json = r#"{"TransactionType":"EscrowCreate","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","SourceTag":11747,"Amount":"10000","Destination":"rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW","DestinationTag":23480,"CancelAfter":533257958,"FinishAfter":533171558,"Condition":"A0258020E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855810100"}"#;

        let txn_as_string = serde_json::to_string(&default_txn).unwrap();
        let txn_json = txn_as_string.as_str();

        assert_eq!(txn_json, default_json);
    }

    #[test]
    fn test_deserialize() {
        let default_txn = EscrowCreate::new(
            "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
            Amount::Xrp(Borrowed("10000")),
            "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW",
            None,
            None,
            None,
            None,
            None,
            Some(11747),
            None,
            None,
            None,
            None,
            Some(23480),
            Some(533257958),
            Some(533171558),
            Some("A0258020E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855810100"),
        );
        let default_json = r#"{"TransactionType":"EscrowCreate","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Amount":"10000","Destination":"rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW","CancelAfter":533257958,"FinishAfter":533171558,"Condition":"A0258020E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855810100","DestinationTag":23480,"SourceTag":11747}"#;

        let txn_as_obj: EscrowCreate = serde_json::from_str(default_json).unwrap();

        assert_eq!(txn_as_obj, default_txn);
    }
}
