use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{
    exceptions::{NFTokenAcceptOfferException, XRPLModelException, XRPLTransactionException},
    model::Model,
    Amount, Memo, NFTokenAcceptOfferError, Signer, Transaction, TransactionType,
};

/// Accept offers to buy or sell an NFToken.
///
/// See NFTokenAcceptOffer:
/// `<https://xrpl.org/nftokenacceptoffer.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
    /// The custom fields for the NFTokenAcceptOffer model.
    ///
    /// See NFTokenAcceptOffer fields:
    /// `<https://xrpl.org/nftokenacceptoffer.html#nftokenacceptoffer-fields>`
    #[serde(rename = "NFTokenSellOffer")]
    pub nftoken_sell_offer: Option<&'a str>,
    #[serde(rename = "NFTokenBuyOffer")]
    pub nftoken_buy_offer: Option<&'a str>,
    #[serde(rename = "NFTokenBrokerFee")]
    pub nftoken_broker_fee: Option<Amount>,
}

impl<'a> Default for NFTokenAcceptOffer<'a> {
    fn default() -> Self {
        Self {
            transaction_type: TransactionType::NFTokenAcceptOffer,
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
            nftoken_sell_offer: Default::default(),
            nftoken_buy_offer: Default::default(),
            nftoken_broker_fee: Default::default(),
        }
    }
}

impl<'a> Model for NFTokenAcceptOffer<'a> {
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

impl<'a> Transaction for NFTokenAcceptOffer<'a> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl<'a> NFTokenAcceptOfferError for NFTokenAcceptOffer<'a> {
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

impl<'a> NFTokenAcceptOffer<'a> {
    fn new(
        account: &'a str,
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
        nftoken_sell_offer: Option<&'a str>,
        nftoken_buy_offer: Option<&'a str>,
        nftoken_broker_fee: Option<Amount>,
    ) -> Self {
        Self {
            transaction_type: TransactionType::NFTokenAcceptOffer,
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
            nftoken_sell_offer,
            nftoken_buy_offer,
            nftoken_broker_fee,
        }
    }
}

#[cfg(test)]
mod test_nftoken_accept_offer_error {
    use alloc::borrow::Cow;

    use crate::models::{
        exceptions::{NFTokenAcceptOfferException, XRPLModelException, XRPLTransactionException},
        Amount, Model, TransactionType,
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
            nftoken_broker_fee: Some(Amount::Xrp(Cow::Borrowed("100"))),
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
            nftoken_broker_fee: Some(Amount::Xrp(Cow::Borrowed("0"))),
        };
        let expected_error = XRPLModelException::XRPLTransactionError(
            XRPLTransactionException::NFTokenAcceptOfferError(
                NFTokenAcceptOfferException::InvalidBrokerFeeMustBeGreaterZero,
            ),
        );
        assert_eq!(nftoken_accept_offer.validate(), Err(expected_error));
    }
}

#[cfg(test)]
mod test_serde {
    use alloc::vec;

    use super::*;

    #[test]
    fn test_serialize() {
        let default_txn = NFTokenAcceptOffer::new(
            "r9spUPhPBfB6kQeF6vPhwmtFwRhBh2JUCG",
            Some("12"),
            Some(68549302),
            Some(75447550),
            None,
            None,
            None,
            None,
            None,
            Some(vec![Memo::new(
                Some("61356534373538372D633134322D346663382D616466362D393666383562356435386437"),
                None,
                None,
            )]),
            None,
            Some("68CD1F6F906494EA08C9CB5CAFA64DFA90D4E834B7151899B73231DE5A0C3B77"),
            None,
            None,
        );
        let default_json = r#"{"TransactionType":"NFTokenAcceptOffer","Account":"r9spUPhPBfB6kQeF6vPhwmtFwRhBh2JUCG","Fee":"12","Sequence":68549302,"LastLedgerSequence":75447550,"Memos":[{"Memo":{"MemoData":"61356534373538372D633134322D346663382D616466362D393666383562356435386437","MemoFormat":null,"MemoType":null}}],"NFTokenSellOffer":"68CD1F6F906494EA08C9CB5CAFA64DFA90D4E834B7151899B73231DE5A0C3B77"}"#;

        let txn_as_string = serde_json::to_string(&default_txn).unwrap();
        let txn_json = txn_as_string.as_str();

        assert_eq!(txn_json, default_json);
    }

    #[test]
    fn test_deserialize() {
        let default_txn = NFTokenAcceptOffer::new(
            "r9spUPhPBfB6kQeF6vPhwmtFwRhBh2JUCG",
            Some("12"),
            Some(68549302),
            Some(75447550),
            None,
            None,
            None,
            None,
            None,
            Some(vec![Memo::new(
                Some("61356534373538372D633134322D346663382D616466362D393666383562356435386437"),
                None,
                None,
            )]),
            None,
            Some("68CD1F6F906494EA08C9CB5CAFA64DFA90D4E834B7151899B73231DE5A0C3B77"),
            None,
            None,
        );
        let default_json = r#"{"TransactionType":"NFTokenAcceptOffer","Account":"r9spUPhPBfB6kQeF6vPhwmtFwRhBh2JUCG","Fee":"12","LastLedgerSequence":75447550,"Memos":[{"Memo":{"MemoData":"61356534373538372D633134322D346663382D616466362D393666383562356435386437","MemoFormat":null,"MemoType":null}}],"NFTokenSellOffer":"68CD1F6F906494EA08C9CB5CAFA64DFA90D4E834B7151899B73231DE5A0C3B77","Sequence":68549302}"#;

        let txn_as_obj: NFTokenAcceptOffer = serde_json::from_str(default_json).unwrap();

        assert_eq!(txn_as_obj, default_txn);
    }
}
