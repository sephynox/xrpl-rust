use alloc::borrow::Cow;
use alloc::vec::Vec;
use anyhow::Result;
use core::convert::TryInto;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;
use strum_macros::{AsRefStr, Display, EnumIter};

use crate::models::{
    model::Model,
    transactions::{Flag, Memo, Signer, Transaction, TransactionType},
};

use crate::models::amount::exceptions::XRPLAmountException;
use crate::models::amount::{Amount, XRPAmount};
use crate::models::transactions::XRPLNFTokenCreateOfferException;
use crate::Err;

use super::{CommonFields, FlagCollection};

/// Transactions of the NFTokenCreateOffer type support additional values
/// in the Flags field. This enum represents those options.
///
/// See NFTokenCreateOffer flags:
/// `<https://xrpl.org/nftokencreateoffer.html#nftokencreateoffer-flags>`
#[derive(
    Debug, Eq, PartialEq, Clone, Serialize_repr, Deserialize_repr, Display, AsRefStr, EnumIter,
)]
#[repr(u32)]
pub enum NFTokenCreateOfferFlag {
    /// If enabled, indicates that the offer is a sell offer.
    /// Otherwise, it is a buy offer.
    TfSellOffer = 0x00000001,
}

/// Creates either a new Sell offer for an NFToken owned by
/// the account executing the transaction, or a new Buy
/// offer for an NFToken owned by another account.
///
/// See NFTokenCreateOffer:
/// `<https://xrpl.org/nftokencreateoffer.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NFTokenCreateOffer<'a> {
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(flatten)]
    pub common_fields: CommonFields<'a, NFTokenCreateOfferFlag>,
    /// The custom fields for the NFTokenCreateOffer model.
    ///
    /// See NFTokenCreateOffer fields:
    /// `<https://xrpl.org/nftokencreateoffer.html#nftokencreateoffer-fields>`
    #[serde(rename = "NFTokenID")]
    pub nftoken_id: Cow<'a, str>,
    pub amount: Amount<'a>,
    pub owner: Option<Cow<'a, str>>,
    pub expiration: Option<u32>,
    pub destination: Option<Cow<'a, str>>,
}

impl<'a: 'static> Model for NFTokenCreateOffer<'a> {
    fn get_errors(&self) -> Result<()> {
        match self._get_amount_error() {
            Err(error) => Err!(error),
            Ok(_no_error) => match self._get_destination_error() {
                Err(error) => Err!(error),
                Ok(_no_error) => match self._get_owner_error() {
                    Err(error) => Err!(error),
                    Ok(_no_error) => Ok(()),
                },
            },
        }
    }
}

impl<'a> Transaction<NFTokenCreateOfferFlag> for NFTokenCreateOffer<'a> {
    fn has_flag(&self, flag: &NFTokenCreateOfferFlag) -> bool {
        self.common_fields.has_flag(flag)
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.common_fields.transaction_type.clone()
    }
}

impl<'a> NFTokenCreateOfferError for NFTokenCreateOffer<'a> {
    fn _get_amount_error(&self) -> Result<()> {
        let amount_into_decimal: Result<Decimal, XRPLAmountException> =
            self.amount.clone().try_into();
        match amount_into_decimal {
            Ok(amount) => {
                if !self.has_flag(&NFTokenCreateOfferFlag::TfSellOffer) && amount.is_zero() {
                    Err!(XRPLNFTokenCreateOfferException::ValueZero {
                        field: "amount".into(),
                        resource: "".into(),
                    })
                } else {
                    Ok(())
                }
            }
            Err(decimal_error) => {
                Err!(decimal_error)
            }
        }
    }

    fn _get_destination_error(&self) -> Result<(), XRPLNFTokenCreateOfferException> {
        if let Some(destination) = self.destination.clone() {
            if destination == self.common_fields.account {
                Err(XRPLNFTokenCreateOfferException::ValueEqualsValue {
                    field1: "destination".into(),
                    field2: "account".into(),
                    resource: "".into(),
                })
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    fn _get_owner_error(&self) -> Result<(), XRPLNFTokenCreateOfferException> {
        if let Some(owner) = self.owner.clone() {
            if self.has_flag(&NFTokenCreateOfferFlag::TfSellOffer) {
                Err(XRPLNFTokenCreateOfferException::IllegalOption {
                    field: "owner".into(),
                    context: "NFToken sell offers".into(),
                    resource: "".into(),
                })
            } else if owner == self.common_fields.account {
                Err(XRPLNFTokenCreateOfferException::ValueEqualsValue {
                    field1: "owner".into(),
                    field2: "account".into(),
                    resource: "".into(),
                })
            } else {
                Ok(())
            }
        } else if !self.has_flag(&NFTokenCreateOfferFlag::TfSellOffer) {
            Err(XRPLNFTokenCreateOfferException::OptionRequired {
                field: "owner".into(),
                context: "NFToken buy offers".into(),
                resource: "".into(),
            })
        } else {
            Ok(())
        }
    }
}

impl<'a> NFTokenCreateOffer<'a> {
    pub fn new(
        account: Cow<'a, str>,
        account_txn_id: Option<Cow<'a, str>>,
        fee: Option<XRPAmount<'a>>,
        flags: Option<FlagCollection<NFTokenCreateOfferFlag>>,
        last_ledger_sequence: Option<u32>,
        memos: Option<Vec<Memo>>,
        sequence: Option<u32>,
        signers: Option<Vec<Signer<'a>>>,
        source_tag: Option<u32>,
        ticket_sequence: Option<u32>,
        amount: Amount<'a>,
        nftoken_id: Cow<'a, str>,
        destination: Option<Cow<'a, str>>,
        expiration: Option<u32>,
        owner: Option<Cow<'a, str>>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                account,
                transaction_type: TransactionType::NFTokenCreateOffer,
                account_txn_id,
                fee,
                flags,
                last_ledger_sequence,
                memos,
                sequence,
                signers,
                source_tag,
                ticket_sequence,
            },
            nftoken_id,
            amount,
            owner,
            expiration,
            destination,
        }
    }
}

pub trait NFTokenCreateOfferError {
    fn _get_amount_error(&self) -> Result<()>;
    fn _get_destination_error(&self) -> Result<(), XRPLNFTokenCreateOfferException>;
    fn _get_owner_error(&self) -> Result<(), XRPLNFTokenCreateOfferException>;
}

#[cfg(test)]
mod test_nftoken_create_offer_error {
    use alloc::string::ToString;
    use alloc::vec;

    use crate::models::{
        amount::{Amount, XRPAmount},
        Model,
    };

    use super::*;

    #[test]
    fn test_amount_error() {
        let nftoken_create_offer = NFTokenCreateOffer::new(
            "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb".into(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Amount::XRPAmount(XRPAmount::from("0")),
            "".into(),
            None,
            None,
            None,
        );

        assert_eq!(
            nftoken_create_offer
                .validate()
                .unwrap_err()
                .to_string()
                .as_str(),
            "The value of the field `amount` is not allowed to be zero. For more information see: "
        );
    }

    #[test]
    fn test_destination_error() {
        let nftoken_create_offer = NFTokenCreateOffer::new(
            "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb".into(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Amount::XRPAmount(XRPAmount::from("1")),
            "".into(),
            Some("rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb".into()),
            None,
            None,
        );

        assert_eq!(
            nftoken_create_offer.validate().unwrap_err().to_string().as_str(),
            "The value of the field `destination` is not allowed to be the same as the value of the field `account`. For more information see: "
        );
    }

    #[test]
    fn test_owner_error() {
        let mut nftoken_create_offer = NFTokenCreateOffer::new(
            "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb".into(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Amount::XRPAmount(XRPAmount::from("1")),
            "".into(),
            None,
            None,
            Some("rLSn6Z3T8uCxbcd1oxwfGQN1Fdn5CyGujK".into()),
        );
        let sell_flag = vec![NFTokenCreateOfferFlag::TfSellOffer];
        nftoken_create_offer.common_fields.flags = Some(sell_flag.into());

        assert_eq!(
            nftoken_create_offer.validate().unwrap_err().to_string().as_str(),
            "The optional field `owner` is not allowed to be defined for NFToken sell offers. For more information see: "
        );

        nftoken_create_offer.common_fields.flags = None;
        nftoken_create_offer.owner = None;

        assert_eq!(
            nftoken_create_offer.validate().unwrap_err().to_string().as_str(),
            "The optional field `owner` is required to be defined for NFToken buy offers. For more information see: "
        );

        nftoken_create_offer.owner = Some("rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb".into());

        assert_eq!(
            nftoken_create_offer.validate().unwrap_err().to_string().as_str(),
            "The value of the field `owner` is not allowed to be the same as the value of the field `account`. For more information see: "
        );
    }
}

#[cfg(test)]
mod test_serde {
    use crate::models::amount::XRPAmount;
    use alloc::vec;

    use super::*;

    #[test]
    fn test_serialize() {
        let default_txn = NFTokenCreateOffer::new(
            "rs8jBmmfpwgmrSPgwMsh7CvKRmRt1JTVSX".into(),
            None,
            None,
            Some(vec![NFTokenCreateOfferFlag::TfSellOffer].into()),
            None,
            None,
            None,
            None,
            None,
            None,
            Amount::XRPAmount(XRPAmount::from("1000000")),
            "000100001E962F495F07A990F4ED55ACCFEEF365DBAA76B6A048C0A200000007".into(),
            None,
            None,
            None,
        );
        let default_json = r#"{"TransactionType":"NFTokenCreateOffer","Account":"rs8jBmmfpwgmrSPgwMsh7CvKRmRt1JTVSX","Flags":1,"NFTokenID":"000100001E962F495F07A990F4ED55ACCFEEF365DBAA76B6A048C0A200000007","Amount":"1000000"}"#;

        let txn_as_string = serde_json::to_string(&default_txn).unwrap();
        let txn_json = txn_as_string.as_str();

        assert_eq!(txn_json, default_json);
    }

    #[test]
    fn test_deserialize() {
        let default_txn = NFTokenCreateOffer::new(
            "rs8jBmmfpwgmrSPgwMsh7CvKRmRt1JTVSX".into(),
            None,
            None,
            Some(vec![NFTokenCreateOfferFlag::TfSellOffer].into()),
            None,
            None,
            None,
            None,
            None,
            None,
            Amount::XRPAmount(XRPAmount::from("1000000")),
            "000100001E962F495F07A990F4ED55ACCFEEF365DBAA76B6A048C0A200000007".into(),
            None,
            None,
            None,
        );
        let default_json = r#"{"TransactionType":"NFTokenCreateOffer","Account":"rs8jBmmfpwgmrSPgwMsh7CvKRmRt1JTVSX","NFTokenID":"000100001E962F495F07A990F4ED55ACCFEEF365DBAA76B6A048C0A200000007","Amount":"1000000","Flags":1}"#;

        let txn_as_obj: NFTokenCreateOffer = serde_json::from_str(default_json).unwrap();

        assert_eq!(txn_as_obj, default_txn);
    }
}
