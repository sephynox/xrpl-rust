use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;
use strum_macros::{AsRefStr, Display, EnumIter};

use crate::{
    constants::{MAX_TRANSFER_FEE, MAX_URI_LENGTH},
    models::{
        exceptions::{NFTokenMintException, XRPLModelException, XRPLTransactionException},
        model::Model,
        Flag, Memo, NFTokenMintError, Signer, Transaction, TransactionType,
    },
};

use super::flags_serde;

/// Transactions of the NFTokenMint type support additional values
/// in the Flags field. This enum represents those options.
///
/// See NFTokenMint flags:
/// `<https://xrpl.org/nftokenmint.html#nftokenmint-flags>`
#[derive(
    Debug, Eq, PartialEq, Clone, Serialize_repr, Deserialize_repr, Display, AsRefStr, EnumIter,
)]
#[repr(u32)]
pub enum NFTokenMintFlag {
    /// Allow the issuer (or an entity authorized by the issuer) to
    /// destroy the minted NFToken. (The NFToken's owner can always do so.)
    TfBurnable = 0x00000001,
    /// The minted NFToken can only be bought or sold for XRP.
    /// This can be desirable if the token has a transfer fee and the issuer
    /// does not want to receive fees in non-XRP currencies.
    TfOnlyXRP = 0x00000002,
    /// The minted NFToken can be transferred to others. If this flag is not
    /// enabled, the token can still be transferred from or to the issuer.
    TfTransferable = 0x00000008,
}

/// The NFTokenMint transaction creates a non-fungible token and adds it to
/// the relevant NFTokenPage object of the NFTokenMinter as an NFToken object.
///
/// See NFTokenMint:
/// `<https://xrpl.org/nftokenmint.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct NFTokenMint<'a> {
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::nftoken_mint")]
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
    #[serde(with = "flags_serde")]
    pub flags: Option<Vec<NFTokenMintFlag>>,
    /// Additional arbitrary information used to identify this transaction.
    pub memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    pub signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the NFTokenMint model.
    ///
    /// See NFTokenMint fields:
    /// `<https://xrpl.org/nftokenmint.html#nftokenmint-fields>`
    pub nftoken_taxon: u32,
    pub issuer: Option<&'a str>,
    pub transfer_fee: Option<u32>,
    pub uri: Option<&'a str>,
}

impl Model for NFTokenMint<'static> {
    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self._get_issuer_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::NFTokenMintError(error),
            )),
            Ok(_no_error) => match self._get_transfer_fee_error() {
                Err(error) => Err(XRPLModelException::XRPLTransactionError(
                    XRPLTransactionException::NFTokenMintError(error),
                )),
                Ok(_no_error) => match self._get_uri_error() {
                    Err(error) => Err(XRPLModelException::XRPLTransactionError(
                        XRPLTransactionException::NFTokenMintError(error),
                    )),
                    Ok(_no_error) => Ok(()),
                },
            },
        }
    }
}

impl From<&NFTokenMint<'static>> for u32 {
    fn from(val: &NFTokenMint<'static>) -> Self {
        val.flags
            .as_ref()
            .unwrap_or(&Vec::new())
            .iter()
            .fold(0, |collect, flag| match flag {
                NFTokenMintFlag::TfBurnable => collect + 0x00000001,
                NFTokenMintFlag::TfOnlyXRP => collect + 0x00000002,
                NFTokenMintFlag::TfTransferable => collect + 0x00000008,
            })
    }
}

impl Transaction for NFTokenMint<'static> {
    fn has_flag(&self, flag: &Flag) -> bool {
        let mut flags = &Vec::new();

        if let Some(flag_set) = self.flags.as_ref() {
            flags = flag_set;
        }

        match flag {
            Flag::NFTokenMint(nftoken_mint_flag) => match nftoken_mint_flag {
                NFTokenMintFlag::TfBurnable => flags.contains(&NFTokenMintFlag::TfBurnable),
                NFTokenMintFlag::TfOnlyXRP => flags.contains(&NFTokenMintFlag::TfOnlyXRP),
                NFTokenMintFlag::TfTransferable => flags.contains(&NFTokenMintFlag::TfTransferable),
            },
            _ => false,
        }
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl NFTokenMintError for NFTokenMint<'static> {
    fn _get_issuer_error(&self) -> Result<(), NFTokenMintException> {
        match self.issuer {
            Some(issuer) => match issuer == self.account {
                true => Err(NFTokenMintException::InvalidIssuerMustNotEqualAccount),
                false => Ok(()),
            },
            None => Ok(()),
        }
    }

    fn _get_transfer_fee_error(&self) -> Result<(), NFTokenMintException> {
        match self.transfer_fee {
            Some(transfer_fee) => match transfer_fee > MAX_TRANSFER_FEE {
                true => Err(NFTokenMintException::InvalidTransferFeeTooHigh {
                    max: MAX_TRANSFER_FEE,
                    found: transfer_fee,
                }),
                false => Ok(()),
            },
            None => Ok(()),
        }
    }

    fn _get_uri_error(&self) -> Result<(), NFTokenMintException> {
        match self.uri {
            Some(uri) => match uri.len() > MAX_URI_LENGTH {
                true => Err(NFTokenMintException::InvalidURITooLong {
                    max: MAX_URI_LENGTH,
                    found: uri.len(),
                }),
                false => Ok(()),
            },
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod test_nftoken_mint_error {
    use crate::models::{
        exceptions::{NFTokenMintException, XRPLModelException, XRPLTransactionException},
        Model, TransactionType,
    };

    use super::NFTokenMint;

    #[test]
    fn test_issuer_error() {
        let nftoken_mint = NFTokenMint {
            transaction_type: TransactionType::NFTokenMint,
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
            nftoken_taxon: 0,
            issuer: Some("rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb"),
            transfer_fee: None,
            uri: None,
        };
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::NFTokenMintError(
                NFTokenMintException::InvalidIssuerMustNotEqualAccount,
            ));
        assert_eq!(nftoken_mint.validate(), Err(expected_error));
    }

    #[test]
    fn test_transfer_fee_error() {
        let nftoken_mint = NFTokenMint {
            transaction_type: TransactionType::NFTokenMint,
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
            nftoken_taxon: 0,
            issuer: None,
            transfer_fee: Some(50001),
            uri: None,
        };
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::NFTokenMintError(
                NFTokenMintException::InvalidTransferFeeTooHigh {
                    max: 50000,
                    found: 50001,
                },
            ));
        assert_eq!(nftoken_mint.validate(), Err(expected_error));
    }

    #[test]
    fn test_uri_error() {
        let nftoken_mint = NFTokenMint {
            transaction_type: TransactionType::NFTokenMint,
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
            nftoken_taxon: 0,
            issuer: None,
            transfer_fee: None,
            uri: Some("wss://xrplcluster.com/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        };
        let expected_error = XRPLModelException::XRPLTransactionError(
            XRPLTransactionException::NFTokenMintError(NFTokenMintException::InvalidURITooLong {
                max: 512,
                found: 513,
            }),
        );
        assert_eq!(nftoken_mint.validate(), Err(expected_error));
    }
}
