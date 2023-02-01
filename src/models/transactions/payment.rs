use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;
use strum_macros::{AsRefStr, Display, EnumIter};

use crate::models::{
    exceptions::{PaymentException, XRPLModelException, XRPLTransactionException},
    model::Model,
    Amount, Flag, Memo, PathStep, PaymentError, Signer, Transaction, TransactionType,
};

use crate::_serde::txn_flags;

/// Transactions of the Payment type support additional values
/// in the Flags field. This enum represents those options.
///
/// See Payment flags:
/// `<https://xrpl.org/payment.html#payment-flags>`
#[derive(
    Debug, Eq, PartialEq, Clone, Serialize_repr, Deserialize_repr, Display, AsRefStr, EnumIter,
)]
#[repr(u32)]
pub enum PaymentFlag {
    /// Do not use the default path; only use paths included in the Paths field.
    /// This is intended to force the transaction to take arbitrage opportunities.
    /// Most clients do not need this.
    TfNoDirectRipple = 0x00010000,
    /// If the specified Amount cannot be sent without spending more than SendMax,
    /// reduce the received amount instead of failing outright.
    /// See Partial Payments for more details.
    TfPartialPayment = 0x00020000,
    /// Only take paths where all the conversions have an input:output ratio that
    /// is equal or better than the ratio of Amount:SendMax.
    /// See Limit Quality for details.
    TfLimitQuality = 0x00040000,
}

/// Transfers value from one account to another.
///
/// See Payment:
/// `<https://xrpl.org/payment.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Payment<'a> {
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::payment")]
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
    #[serde(with = "txn_flags")]
    pub flags: Option<Vec<PaymentFlag>>,
    /// Additional arbitrary information used to identify this transaction.
    pub memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    pub signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the Payment model.
    ///
    /// See Payment fields:
    /// `<https://xrpl.org/payment.html#payment-fields>`
    pub amount: Amount,
    pub destination: &'a str,
    pub destination_tag: Option<u32>,
    pub invoice_id: Option<u32>,
    pub paths: Option<Vec<Vec<PathStep<'a>>>>,
    pub send_max: Option<Amount>,
    pub deliver_min: Option<Amount>,
}

impl<'a> Default for Payment<'a> {
    fn default() -> Self {
        Self {
            transaction_type: TransactionType::Payment,
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
            invoice_id: Default::default(),
            paths: Default::default(),
            send_max: Default::default(),
            deliver_min: Default::default(),
        }
    }
}

impl<'a> Model for Payment<'a> {
    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self._get_xrp_transaction_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::PaymentError(error),
            )),
            Ok(_no_error) => match self._get_partial_payment_error() {
                Err(error) => Err(XRPLModelException::XRPLTransactionError(
                    XRPLTransactionException::PaymentError(error),
                )),
                Ok(_no_error) => match self._get_exchange_error() {
                    Err(error) => Err(XRPLModelException::XRPLTransactionError(
                        XRPLTransactionException::PaymentError(error),
                    )),
                    Ok(_no_error) => Ok(()),
                },
            },
        }
    }
}

impl<'a> Transaction for Payment<'a> {
    fn has_flag(&self, flag: &Flag) -> bool {
        let mut flags = &Vec::new();

        if let Some(flag_set) = self.flags.as_ref() {
            flags = flag_set;
        }

        match flag {
            Flag::Payment(payment_flag) => match payment_flag {
                PaymentFlag::TfLimitQuality => flags.contains(&PaymentFlag::TfLimitQuality),
                PaymentFlag::TfNoDirectRipple => flags.contains(&PaymentFlag::TfNoDirectRipple),
                PaymentFlag::TfPartialPayment => flags.contains(&PaymentFlag::TfPartialPayment),
            },
            _ => false,
        }
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl<'a> PaymentError for Payment<'a> {
    fn _get_xrp_transaction_error(&self) -> Result<(), PaymentException> {
        match self.amount.is_xrp() && self.send_max.is_none() {
            true => match self.paths.is_some() {
                true => Err(PaymentException::InvalidXRPtoXRPPaymentsCannotContainPaths),
                false => match self.account == self.destination {
                    true => Err(
                        PaymentException::InvalidDestinationMustNotEqualAccountForXRPtoXRPPayments,
                    ),
                    false => Ok(()),
                },
            },
            false => Ok(()),
        }
    }

    fn _get_partial_payment_error(&self) -> Result<(), PaymentException> {
        match self.send_max.as_ref() {
            Some(send_max) => match !self.has_flag(&Flag::Payment(PaymentFlag::TfPartialPayment)) {
                true => match send_max.is_xrp() && self.amount.is_xrp() {
                    true => Err(
                        PaymentException::InvalidSendMaxMustNotBeSetForXRPtoXRPNonPartialPayments,
                    ),
                    false => Ok(()),
                },
                false => Ok(()),
            },
            None => match self.has_flag(&Flag::Payment(PaymentFlag::TfPartialPayment)) {
                true => Err(PaymentException::InvalidSendMaxMustBeSetForPartialPayments),
                false => match self.deliver_min.as_ref() {
                    Some(_deliver_min) => {
                        Err(PaymentException::InvalidDeliverMinMustNotBeSetForNonPartialPayments)
                    }
                    None => Ok(()),
                },
            },
        }
    }

    fn _get_exchange_error(&self) -> Result<(), PaymentException> {
        match self.account == self.destination {
            true => match self.send_max.as_ref() {
                Some(_send_max) => Ok(()),
                None => Err(PaymentException::InvalidSendMaxMustBeSetForExchanges),
            },
            false => Ok(()),
        }
    }
}

impl<'a> Payment<'a> {
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
        flags: Option<Vec<PaymentFlag>>,
        memos: Option<Vec<Memo<'a>>>,
        signers: Option<Vec<Signer<'a>>>,
        destination_tag: Option<u32>,
        invoice_id: Option<u32>,
        paths: Option<Vec<Vec<PathStep<'a>>>>,
        send_max: Option<Amount>,
        deliver_min: Option<Amount>,
    ) -> Self {
        Self {
            transaction_type: TransactionType::Payment,
            account,
            fee,
            sequence,
            last_ledger_sequence,
            account_txn_id,
            signing_pub_key,
            source_tag,
            ticket_sequence,
            txn_signature,
            flags,
            memos,
            signers,
            amount,
            destination,
            destination_tag,
            invoice_id,
            paths,
            send_max,
            deliver_min,
        }
    }
}

#[cfg(test)]
mod test_payment_error {
    use alloc::{borrow::Cow, vec};

    use crate::models::{
        exceptions::{PaymentException, XRPLModelException, XRPLTransactionException},
        Amount, Model, PathStep, PaymentFlag, TransactionType,
    };

    use super::Payment;

    #[test]
    fn test_xrp_to_xrp_error() {
        let mut payment = Payment {
            transaction_type: TransactionType::Payment,
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
            amount: Amount::Xrp(Cow::Borrowed("1000000")),
            destination: "rLSn6Z3T8uCxbcd1oxwfGQN1Fdn5CyGujK",
            destination_tag: None,
            invoice_id: None,
            paths: Some(vec![vec![PathStep {
                account: Some("rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B"),
                currency: None,
                issuer: None,
                r#type: None,
                type_hex: None,
            }]]),
            send_max: None,
            deliver_min: None,
        };
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::PaymentError(
                PaymentException::InvalidXRPtoXRPPaymentsCannotContainPaths,
            ));
        assert_eq!(payment.validate(), Err(expected_error));

        payment.paths = None;
        payment.send_max = Some(Amount::Xrp(Cow::Borrowed("99999")));
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::PaymentError(
                PaymentException::InvalidSendMaxMustNotBeSetForXRPtoXRPNonPartialPayments,
            ));
        assert_eq!(payment.validate(), Err(expected_error));

        payment.send_max = None;
        payment.destination = "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb";
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::PaymentError(
                PaymentException::InvalidDestinationMustNotEqualAccountForXRPtoXRPPayments,
            ));
        assert_eq!(payment.validate(), Err(expected_error));
    }

    #[test]
    fn test_partial_payments_eror() {
        let mut payment = Payment {
            transaction_type: TransactionType::Payment,
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
            amount: Amount::Xrp(Cow::Borrowed("1000000")),
            destination: "rLSn6Z3T8uCxbcd1oxwfGQN1Fdn5CyGujK",
            destination_tag: None,
            invoice_id: None,
            paths: None,
            send_max: None,
            deliver_min: None,
        };
        payment.flags = Some(vec![PaymentFlag::TfPartialPayment]);
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::PaymentError(
                PaymentException::InvalidSendMaxMustBeSetForPartialPayments,
            ));
        assert_eq!(payment.validate(), Err(expected_error));

        payment.flags = None;
        payment.deliver_min = Some(Amount::Xrp(Cow::Borrowed("99999")));
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::PaymentError(
                PaymentException::InvalidDeliverMinMustNotBeSetForNonPartialPayments,
            ));
        assert_eq!(payment.validate(), Err(expected_error));
    }

    #[test]
    fn test_exchange_error() {
        let payment = Payment {
            transaction_type: TransactionType::Payment,
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
            amount: Amount::IssuedCurrency {
                value: Cow::Borrowed("10"),
                currency: Cow::Borrowed("USD"),
                issuer: Cow::Borrowed("rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B"),
            },
            destination: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            destination_tag: None,
            invoice_id: None,
            paths: None,
            send_max: None,
            deliver_min: None,
        };
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::PaymentError(
                PaymentException::InvalidSendMaxMustBeSetForExchanges,
            ));
        assert_eq!(payment.validate(), Err(expected_error));
    }
}
