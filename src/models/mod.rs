//! Top-level modules for the models package.

pub mod exceptions;
pub mod requests;
pub mod utils;
use alloc::borrow::Cow::Borrowed;
pub use requests::*;

use alloc::borrow::Cow;
use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;
use strum_macros::{Display, EnumIter};

/// Represents the different options for the `method`
/// field in a request.
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Display, EnumIter)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum RequestMethod {
    // Account methods
    AccountChannels,
    AccountCurrencies,
    AccountInfo,
    AccountLines,
    AccountObjects,
    AccountOffers,
    AccountTx,
    GatewayBalances,
    NoRippleCheck,

    // Transaction methods
    Sign,
    SignFor,
    Submit,
    SubmitMultisigned,
    TransactionEntry,
    Tx,

    // Channel methods
    ChannelAuthorize,
    ChannelVerify,

    // Path methods
    BookOffers,
    DepositAuthorized,
    PathFind,
    RipplePathFind,

    // Ledger methods
    Ledger,
    LedgerClosed,
    LedgerCurrent,
    LedgerData,
    LedgerEntry,

    // Subscription methods
    Subscribe,
    Unsubscribe,

    // Server info methods
    Fee,
    Manifest,
    ServerInfo,
    ServerState,

    // Utility methods
    Ping,
    Random,
}

/// Transactions of the TrustSet type support additional values
/// in the Flags field. This enum represents those options.
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Display, AsRefStr)]
pub enum TrustSetFlag {
    TfSetAuth,
    TfSetNoRipple,
    TfClearNoRipple,
    TfSetFreeze,
    TfClearFreeze,
}

/// Represents the object types that an AccountObjects
/// Request can ask for.
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum AccountObjectType {
    Check,
    DepositPreauth,
    Escrow,
    Offer,
    PaymentChannel,
    SignerList,
    State,
    Ticket,
}

/// Specifies an amount in an issued currency.
///
/// See Specifying Currency Amounts:
/// `<https://xrpl.org/currency-formats.html#specifying-currency-amounts>`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Currency {
    /// Specifies an amount in an issued currency.
    IssuedCurrency {
        amount: Option<Cow<'static, str>>,
        currency: Cow<'static, str>,
        issuer: Cow<'static, str>,
    },
    /// Specifies an amount in XRP.
    Xrp {
        amount: Option<Cow<'static, str>>,
        currency: Cow<'static, str>,
    },
}

/// Enum containing the different Transaction types.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(tag = "transaction_type")]
pub enum TransactionType {
    AccountDelete,
    AccountSet,
    CheckCancel,
    CheckCash,
    CheckCreate,
    DepositPreauth,
    EscrowCancel,
    EscrowCreate,
    EscrowFinish,
    OfferCancel,
    OfferCreate,
    Payment,
    PaymentChannelClaim,
    PaymentChannelCreate,
    PaymentChannelFund,
    SetRegularKey,
    SignerListSet,
    TrustSet,
}

/// Enum containing the different Psuedo-Transaction types.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(tag = "transaction_type")]
pub enum PseudoTransactionType {
    EnableAmendment,
    SetFee,
    UNLModify,
}

/// Returns a Currency as XRP for the currency, without a value.
pub fn default_xrp_currency() -> Currency {
    Currency::Xrp {
        amount: None,
        currency: Borrowed("XRP"),
    }
}

/// Allows creation of a Model object based on a JSON-like
/// dictionary of keys in the JSON format used by the binary
/// codec, or an actual JSON string representing the same data.
pub trait FromXRPL<T> {
    fn from_xrpl(value: T) -> Self;
}

impl TrustSetFlag {
    fn tf_set_auth() -> u32 {
        0x00010000
    }
    fn tf_set_no_ripple() -> u32 {
        0x00020000
    }
    fn tf_clear_no_ripple() -> u32 {
        0x00040000
    }
    fn tf_set_freeze() -> u32 {
        0x00100000
    }
    fn tf_clear_freeze() -> u32 {
        0x00200000
    }
}

/// TODO Find a better way
impl RequestMethod {
    fn account_channels() -> Self {
        RequestMethod::AccountChannels
    }
    fn account_currencies() -> Self {
        RequestMethod::AccountCurrencies
    }
    fn account_info() -> Self {
        RequestMethod::AccountInfo
    }
    fn account_lines() -> Self {
        RequestMethod::AccountLines
    }
    fn account_objects() -> Self {
        RequestMethod::AccountObjects
    }
    fn account_offers() -> Self {
        RequestMethod::AccountOffers
    }
    fn account_tx() -> Self {
        RequestMethod::AccountTx
    }
    fn book_offers() -> Self {
        RequestMethod::BookOffers
    }
    fn channel_authorize() -> Self {
        RequestMethod::ChannelAuthorize
    }
    fn channel_verify() -> Self {
        RequestMethod::ChannelVerify
    }
    fn deposit_authorization() -> Self {
        RequestMethod::DepositAuthorized
    }
}
