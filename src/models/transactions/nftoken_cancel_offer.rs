use crate::Err;
use alloc::borrow::Cow;
use alloc::vec::Vec;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::amount::XRPAmount;
use crate::models::transactions::{NoFlags, XRPLNFTokenCancelOfferException};
use crate::models::{
    model::Model,
    transactions::{Memo, Signer, Transaction, TransactionType},
};

use super::CommonFields;

/// Cancels existing token offers created using NFTokenCreateOffer.
///
/// See NFTokenCancelOffer:
/// `<https://xrpl.org/nftokencanceloffer.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NFTokenCancelOffer<'a> {
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(flatten)]
    pub common_fields: CommonFields<'a, NoFlags>,
    /// The custom fields for the NFTokenCancelOffer model.
    ///
    /// See NFTokenCancelOffer fields:
    /// `<https://xrpl.org/nftokencanceloffer.html#nftokencanceloffer-fields>`
    /// Lifetime issue
    #[serde(borrow)]
    #[serde(rename = "NFTokenOffers")]
    pub nftoken_offers: Vec<Cow<'a, str>>,
}

impl<'a: 'static> Model for NFTokenCancelOffer<'a> {
    fn get_errors(&self) -> Result<()> {
        match self._get_nftoken_offers_error() {
            Ok(_) => Ok(()),
            Err(error) => Err!(error),
        }
    }
}

impl<'a> Transaction<NoFlags> for NFTokenCancelOffer<'a> {
    fn get_transaction_type(&self) -> TransactionType {
        self.common_fields.transaction_type.clone()
    }
}

impl<'a> NFTokenCancelOfferError for NFTokenCancelOffer<'a> {
    fn _get_nftoken_offers_error(&self) -> Result<(), XRPLNFTokenCancelOfferException> {
        if self.nftoken_offers.is_empty() {
            Err(XRPLNFTokenCancelOfferException::CollectionEmpty {
                field: "nftoken_offers".into(),
                r#type: stringify!(Vec).into(),
                resource: "".into(),
            })
        } else {
            Ok(())
        }
    }
}

impl<'a> NFTokenCancelOffer<'a> {
    pub fn new(
        account: Cow<'a, str>,
        account_txn_id: Option<Cow<'a, str>>,
        fee: Option<XRPAmount<'a>>,
        last_ledger_sequence: Option<u32>,
        memos: Option<Vec<Memo>>,
        sequence: Option<u32>,
        signers: Option<Vec<Signer<'a>>>,
        source_tag: Option<u32>,
        ticket_sequence: Option<u32>,
        nftoken_offers: Vec<Cow<'a, str>>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                account,
                transaction_type: TransactionType::NFTokenCancelOffer,
                account_txn_id,
                fee,
                flags: None,
                last_ledger_sequence,
                memos,
                sequence,
                signers,
                source_tag,
                ticket_sequence,
            },
            nftoken_offers,
        }
    }
}

pub trait NFTokenCancelOfferError {
    fn _get_nftoken_offers_error(&self) -> Result<(), XRPLNFTokenCancelOfferException>;
}

#[cfg(test)]
mod test_nftoken_cancel_offer_error {
    use alloc::string::ToString;
    use alloc::vec::Vec;

    use crate::models::Model;

    use super::*;

    #[test]
    fn test_nftoken_offer_error() {
        let nftoken_cancel_offer = NFTokenCancelOffer::new(
            "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb".into(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Vec::new(),
        );

        assert_eq!(
            nftoken_cancel_offer.validate().unwrap_err().to_string().as_str(),
            "The value of the field `nftoken_offers` is not allowed to be empty (type `Vec`). If the field is optional, define it to be `None`. For more information see: "
        );
    }
}

#[cfg(test)]
mod test_serde {
    use alloc::vec;

    use super::*;

    #[test]
    fn test_serialize() {
        let default_txn = NFTokenCancelOffer::new(
            "ra5nK24KXen9AHvsdFTKHSANinZseWnPcX".into(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            vec!["9C92E061381C1EF37A8CDE0E8FC35188BFC30B1883825042A64309AC09F4C36D".into()],
        );
        let default_json = r#"{"TransactionType":"NFTokenCancelOffer","Account":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","NFTokenOffers":["9C92E061381C1EF37A8CDE0E8FC35188BFC30B1883825042A64309AC09F4C36D"]}"#;

        let txn_as_string = serde_json::to_string(&default_txn).unwrap();
        let txn_json = txn_as_string.as_str();

        assert_eq!(txn_json, default_json);
    }

    #[test]
    fn test_deserialize() {
        let default_txn = NFTokenCancelOffer::new(
            "ra5nK24KXen9AHvsdFTKHSANinZseWnPcX".into(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            vec!["9C92E061381C1EF37A8CDE0E8FC35188BFC30B1883825042A64309AC09F4C36D".into()],
        );
        let default_json = r#"{"TransactionType":"NFTokenCancelOffer","Account":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","NFTokenOffers":["9C92E061381C1EF37A8CDE0E8FC35188BFC30B1883825042A64309AC09F4C36D"]}"#;

        let txn_as_obj: NFTokenCancelOffer = serde_json::from_str(default_json).unwrap();

        assert_eq!(txn_as_obj, default_txn);
    }
}
