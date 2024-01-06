use alloc::borrow::Cow;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;
use strum_macros::{AsRefStr, Display, EnumIter};

use crate::models::{
    model::Model,
    transactions::{Memo, Signer, Transaction, TransactionType},
};

use crate::models::amount::{IssuedCurrencyAmount, XRPAmount};

use super::{CommonFields, FlagCollection};

/// Transactions of the TrustSet type support additional values
/// in the Flags field. This enum represents those options.
///
/// See TrustSet flags:
#[derive(
    Debug, Eq, PartialEq, Clone, Serialize_repr, Deserialize_repr, Display, AsRefStr, EnumIter,
)]
#[repr(u32)]
pub enum TrustSetFlag {
    /// Authorize the other party to hold currency issued by this account.
    /// (No effect unless using the asfRequireAuth AccountSet flag.) Cannot be unset.
    TfSetAuth = 0x00010000,
    /// Enable the No Ripple flag, which blocks rippling between two trust lines
    /// of the same currency if this flag is enabled on both.
    TfSetNoRipple = 0x00020000,
    /// Disable the No Ripple flag, allowing rippling on this trust line.)
    TfClearNoRipple = 0x00040000,
    /// Freeze the trust line.
    TfSetFreeze = 0x00100000,
    /// Unfreeze the trust line.
    TfClearFreeze = 0x00200000,
}

/// Create or modify a trust line linking two accounts.
///
/// See TrustSet:
/// `<https://xrpl.org/trustset.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TrustSet<'a> {
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(flatten)]
    pub common_fields: CommonFields<'a, TrustSetFlag>,
    // The custom fields for the TrustSet model.
    //
    // See TrustSet fields:
    // `<https://xrpl.org/trustset.html#trustset-fields>`
    /// Object defining the trust line to create or modify, in the format of a Currency Amount.
    pub limit_amount: IssuedCurrencyAmount<'a>,
    /// Value incoming balances on this trust line at the ratio of this number per
    /// 1,000,000,000 units. A value of 0 is shorthand for treating balances at face value.
    pub quality_in: Option<u32>,
    /// Value outgoing balances on this trust line at the ratio of this number per
    /// 1,000,000,000 units. A value of 0 is shorthand for treating balances at face value.
    pub quality_out: Option<u32>,
}

impl<'a> Model for TrustSet<'a> {}

impl<'a> Transaction<TrustSetFlag> for TrustSet<'a> {
    fn has_flag(&self, flag: &TrustSetFlag) -> bool {
        self.common_fields.has_flag(flag)
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.common_fields.transaction_type.clone()
    }
}

impl<'a> TrustSet<'a> {
    pub fn new(
        account: Cow<'a, str>,
        account_txn_id: Option<Cow<'a, str>>,
        fee: Option<XRPAmount<'a>>,
        flags: Option<FlagCollection<TrustSetFlag>>,
        last_ledger_sequence: Option<u32>,
        memos: Option<Vec<Memo>>,
        sequence: Option<u32>,
        signers: Option<Vec<Signer<'a>>>,
        source_tag: Option<u32>,
        ticket_sequence: Option<u32>,
        limit_amount: IssuedCurrencyAmount<'a>,
        quality_in: Option<u32>,
        quality_out: Option<u32>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                account,
                transaction_type: TransactionType::TrustSet,
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
            limit_amount,
            quality_in,
            quality_out,
        }
    }
}

#[cfg(test)]
mod test_serde {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_serialize() {
        let default_txn = TrustSet::new(
            "ra5nK24KXen9AHvsdFTKHSANinZseWnPcX".into(),
            None,
            Some("12".into()),
            Some(vec![TrustSetFlag::TfClearNoRipple].into()),
            Some(8007750),
            None,
            Some(12),
            None,
            None,
            None,
            IssuedCurrencyAmount::new(
                "USD".into(),
                "rsP3mgGb2tcYUrxiLFiHJiQXhsziegtwBc".into(),
                "100".into(),
            ),
            None,
            None,
        );
        let default_json = r#"{"TransactionType":"TrustSet","Account":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Fee":"12","Sequence":12,"LastLedgerSequence":8007750,"Flags":262144,"LimitAmount":{"currency":"USD","issuer":"rsP3mgGb2tcYUrxiLFiHJiQXhsziegtwBc","value":"100"}}"#;

        let txn_as_string = serde_json::to_string(&default_txn).unwrap();
        let txn_json = txn_as_string.as_str();

        assert_eq!(txn_json, default_json);
    }

    #[test]
    fn test_deserialize() {
        let default_txn = TrustSet::new(
            "ra5nK24KXen9AHvsdFTKHSANinZseWnPcX".into(),
            None,
            Some("12".into()),
            Some(vec![TrustSetFlag::TfClearNoRipple].into()),
            Some(8007750),
            None,
            Some(12),
            None,
            None,
            None,
            IssuedCurrencyAmount::new(
                "USD".into(),
                "rsP3mgGb2tcYUrxiLFiHJiQXhsziegtwBc".into(),
                "100".into(),
            ),
            None,
            None,
        );
        let default_json = r#"{"TransactionType":"TrustSet","Account":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Fee":"12","Flags":262144,"LastLedgerSequence":8007750,"LimitAmount":{"currency":"USD","issuer":"rsP3mgGb2tcYUrxiLFiHJiQXhsziegtwBc","value":"100"},"Sequence":12}"#;

        let txn_as_obj: TrustSet = serde_json::from_str(default_json).unwrap();

        assert_eq!(txn_as_obj, default_txn);
    }
}
