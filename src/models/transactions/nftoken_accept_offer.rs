use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{
    exceptions::{NFTokenAcceptOfferException, XRPLModelException, XRPLTransactionException},
    model::Model,
    CurrencyAmount, Memo, NFTokenAcceptOfferError, Signer, Transaction, TransactionType,
};

/// Accept offers to buy or sell an NFToken.
///
/// See NFTokenAcceptOffer:
/// `<https://xrpl.org/nftokenacceptoffer.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct NFTokenAcceptOffer<'a> {
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::nftoken_accept_offer")]
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
    flags: Option<u32>,
    /// Additional arbitrary information used to identify this transaction.
    pub memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    pub signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the NFTokenAcceptOffer model.
    ///
    /// See NFTokenAcceptOffer fields:
    /// `<https://xrpl.org/nftokenacceptoffer.html#nftokenacceptoffer-fields>`
    pub nftoken_sell_offer: Option<&'a str>,
    pub nftoken_buy_offer: Option<&'a str>,
    pub nftoken_broker_fee: Option<CurrencyAmount>,
}

impl Model for NFTokenAcceptOffer<'static> {
    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self._get_brokered_mode_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::NFTokenAcceptOfferError(error),
            )),
            Ok(_no_error) => match self._get_nftoken_broker_fee_error() {
                Err(error) => Err(XRPLModelException::XRPLTransactionError(
                    XRPLTransactionException::NFTokenAcceptOfferError(error),
                )),
                Ok(_no_error) => Ok(()),
            },
        }
    }
}

impl Transaction for NFTokenAcceptOffer<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl NFTokenAcceptOfferError for NFTokenAcceptOffer<'static> {
    fn _get_brokered_mode_error(&self) -> Result<(), NFTokenAcceptOfferException> {
        match self.nftoken_broker_fee.as_ref() {
            Some(_nftoken_broker_fee) => match self.nftoken_sell_offer.is_none() && self.nftoken_buy_offer.is_none() {
                true => Err(NFTokenAcceptOfferException::InvalidMustSetEitherNftokenBuyOfferOrNftokenSellOffer),
                false => Ok(()),
            }
            None => Ok(()),
        }
    }
    fn _get_nftoken_broker_fee_error(&self) -> Result<(), NFTokenAcceptOfferException> {
        match self.nftoken_broker_fee.as_ref() {
            Some(nftoken_broker_fee) => match nftoken_broker_fee.get_value_as_u32() == 0 {
                true => Err(NFTokenAcceptOfferException::InvalidBrokerFeeMustBeGreaterZero),
                false => Ok(()),
            },
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod test_nftoken_accept_offer_error {
    use alloc::borrow::Cow;

    use crate::models::{
        exceptions::{NFTokenAcceptOfferException, XRPLModelException, XRPLTransactionException},
        CurrencyAmount, Model, TransactionType,
    };

    use super::NFTokenAcceptOffer;

    #[test]
    fn test_brokered_mode_error() {
        let nftoken_accept_offer = NFTokenAcceptOffer {
            transaction_type: TransactionType::NFTokenAcceptOffer,
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
            nftoken_sell_offer: None,
            nftoken_buy_offer: None,
            nftoken_broker_fee: Some(CurrencyAmount::Xrp(Cow::Borrowed("100"))),
        };
        let expected_error = XRPLModelException::XRPLTransactionError(
            XRPLTransactionException::NFTokenAcceptOfferError(
                NFTokenAcceptOfferException::InvalidMustSetEitherNftokenBuyOfferOrNftokenSellOffer,
            ),
        );
        assert_eq!(nftoken_accept_offer.validate(), Err(expected_error));
    }

    #[test]
    fn test_broker_fee_error() {
        let nftoken_accept_offer = NFTokenAcceptOffer {
            transaction_type: TransactionType::NFTokenAcceptOffer,
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
            nftoken_sell_offer: Some(""),
            nftoken_buy_offer: None,
            nftoken_broker_fee: Some(CurrencyAmount::Xrp(Cow::Borrowed("0"))),
        };
        let expected_error = XRPLModelException::XRPLTransactionError(
            XRPLTransactionException::NFTokenAcceptOfferError(
                NFTokenAcceptOfferException::InvalidBrokerFeeMustBeGreaterZero,
            ),
        );
        assert_eq!(nftoken_accept_offer.validate(), Err(expected_error));
    }
}
