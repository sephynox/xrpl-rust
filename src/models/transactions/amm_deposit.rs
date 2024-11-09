use alloc::{borrow::Cow, vec::Vec};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum_macros::{AsRefStr, Display, EnumIter};

use crate::models::{
    transactions::TransactionType, Amount, Currency, FlagCollection, IssuedCurrencyAmount, Model,
    XRPAmount, XRPLModelException, XRPLModelResult,
};

use super::{CommonFields, Memo, Signer, Transaction};

/// Transactions of the AMMDeposit type support additional values in the Flags field.
/// This enum represents those options.
#[derive(
    Debug, Eq, PartialEq, Clone, Serialize_repr, Deserialize_repr, Display, AsRefStr, EnumIter,
)]
#[repr(u32)]
pub enum AMMDepositFlag {
    TfLpToken = 0x00010000,
    TfSingleAsset = 0x00080000,
    TfTwoAsset = 0x00100000,
    TfOneAssetLpToken = 0x00200000,
    TfLimitLpToken = 0x00400000,
    TfTwoAssetIfEmpty = 0x00800000,
}

/// Deposit funds into an Automated Market Maker (AMM) instance
/// and receive the AMM's liquidity provider tokens (LP Tokens) in exchange.
///
/// You can deposit one or both of the assets in the AMM's pool.
/// If successful, this transaction creates a trust line to the AMM Account (limit 0)
/// to hold the LP Tokens.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AMMDeposit<'a> {
    #[serde(flatten)]
    pub common_fields: CommonFields<'a, AMMDepositFlag>,
    /// The definition for one of the assets in the AMM's pool.
    pub asset: Currency<'a>,
    /// The definition for the other asset in the AMM's pool.
    pub asset2: Currency<'a>,
    /// The amount of one asset to deposit to the AMM.
    /// If present, this must match the type of one of the assets (tokens or XRP)
    /// in the AMM's pool.
    pub amount: Option<Amount<'a>>,
    /// The amount of another asset to add to the AMM.
    /// If present, this must match the type of the other asset in the AMM's pool
    /// and cannot be the same asset as Amount.
    pub amount2: Option<Amount<'a>>,
    /// The maximum effective price, in the deposit asset, to pay
    /// for each LP Token received.
    pub e_price: Option<Amount<'a>>,
    /// How many of the AMM's LP Tokens to buy.
    pub lp_token_out: Option<IssuedCurrencyAmount<'a>>,
}

impl Model for AMMDeposit<'_> {
    fn get_errors(&self) -> XRPLModelResult<()> {
        if self.amount2.is_some() && self.amount.is_none() {
            Err(XRPLModelException::FieldRequiresField {
                field1: "amount2".into(),
                field2: "amount".into(),
            })
        } else if self.e_price.is_some() && self.amount.is_none() {
            Err(XRPLModelException::FieldRequiresField {
                field1: "e_price".into(),
                field2: "amount".into(),
            })
        } else if self.lp_token_out.is_none() && self.amount.is_none() {
            Err(XRPLModelException::ExpectedOneOf(
                ["lp_token_out", "amount"].as_ref().into(),
            ))
        } else {
            Ok(())
        }
    }
}

impl<'a> Transaction<'a, AMMDepositFlag> for AMMDeposit<'a> {
    fn get_common_fields(&self) -> &CommonFields<'_, AMMDepositFlag> {
        &self.common_fields
    }

    fn get_mut_common_fields(&mut self) -> &mut CommonFields<'a, AMMDepositFlag> {
        self.common_fields.get_mut_common_fields()
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.common_fields.get_transaction_type()
    }
}

impl<'a> AMMDeposit<'a> {
    pub fn new(
        account: Cow<'a, str>,
        account_txn_id: Option<Cow<'a, str>>,
        fee: Option<XRPAmount<'a>>,
        flags: Option<FlagCollection<AMMDepositFlag>>,
        last_ledger_sequence: Option<u32>,
        memos: Option<Vec<Memo>>,
        sequence: Option<u32>,
        signers: Option<Vec<Signer<'a>>>,
        source_tag: Option<u32>,
        ticket_sequence: Option<u32>,
        asset: Currency<'a>,
        asset2: Currency<'a>,
        amount: Option<Amount<'a>>,
        amount2: Option<Amount<'a>>,
        e_price: Option<Amount<'a>>,
        lp_token_out: Option<IssuedCurrencyAmount<'a>>,
    ) -> AMMDeposit<'a> {
        AMMDeposit {
            common_fields: CommonFields {
                account,
                transaction_type: TransactionType::AMMDeposit,
                account_txn_id,
                fee,
                flags: flags.unwrap_or_default(),
                last_ledger_sequence,
                memos,
                sequence,
                signers,
                source_tag,
                ticket_sequence,
                network_id: None,
                signing_pub_key: None,
                txn_signature: None,
            },
            asset,
            asset2,
            amount,
            amount2,
            e_price,
            lp_token_out,
        }
    }
}
