//! Top-level modules for the models package.

pub mod exceptions;
pub mod model;
pub mod request_fields;
pub mod requests;
pub mod transactions;
pub mod utils;

pub use model::Model;
pub use requests::*;
pub use transactions::*;

use alloc::borrow::Cow;
use alloc::borrow::Cow::Borrowed;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use strum_macros::AsRefStr;
use strum_macros::{Display, EnumIter};

use self::exceptions::{
    AccountSetException, ChannelAuthorizeException, CheckCashException, DepositPreauthException,
    EscrowCreateException, EscrowFinishException, LedgerEntryException,
    NFTokenAcceptOfferException, NFTokenCancelOfferException, NFTokenCreateOfferException,
    NFTokenMintException, PaymentException, SignAndSubmitException, SignException,
    SignForException, SignerListSetException, UNLModifyException,
};

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

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Display, AsRefStr)]
pub enum Flag {
    AccountSet(AccountSetFlag),
    NFTokenCreateOffer(NFTokenCreateOfferFlag),
    NFTokenMint(NFTokenMintFlag),
    OfferCreate(OfferCreateFlag),
    Payment(PaymentFlag),
    PaymentChannelClaim(PaymentChannelClaimFlag),
    TrustSet(TrustSetFlag),
    EnableAmendment(EnableAmendmentFlag),
}

/// Transactions of the AccountSet type support additional values
/// in the Flags field. This enum represents those options.
///
/// See AccountSet flags:
/// `<https://xrpl.org/accountset.html#accountset-flags>`
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Display, AsRefStr)]
pub enum AccountSetFlag {
    /// Track the ID of this account's most recent transaction
    /// Required for AccountTxnID
    AsfAccountTxnID,
    /// Enable to allow another account to mint non-fungible tokens (NFTokens)
    /// on this account's behalf. Specify the authorized account in the
    /// NFTokenMinter field of the AccountRoot object. This is an experimental
    /// field to enable behavior for NFToken support.
    AsfAuthorizedNFTokenMinter,
    /// Enable rippling on this account's trust lines by default.
    AsfDefaultRipple,
    /// Enable Deposit Authorization on this account.
    /// (Added by the DepositAuth amendment.)
    AsfDepositAuth,
    /// Disallow use of the master key pair. Can only be enabled if the
    /// account has configured another way to sign transactions, such as
    /// a Regular Key or a Signer List.
    AsfDisableMaster,
    /// XRP should not be sent to this account.
    /// (Enforced by client applications, not by rippled)
    AsfDisallowXRP,
    /// Freeze all assets issued by this account.
    AsfGlobalFreeze,
    /// Permanently give up the ability to freeze individual
    /// trust lines or disable Global Freeze. This flag can never
    /// be disabled after being enabled.
    AsfNoFreeze,
    /// Require authorization for users to hold balances issued by
    /// this address. Can only be enabled if the address has no
    /// trust lines connected to it.
    AsfRequireAuth,
    /// Require a destination tag to send transactions to this account.
    AsfRequireDest,
}

/// Transactions of the NFTokenCreateOffer type support additional values
/// in the Flags field. This enum represents those options.
///
/// See NFTokenCreateOffer flags:
/// `<https://xrpl.org/nftokencreateoffer.html#nftokencreateoffer-flags>`
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Display, AsRefStr)]
pub enum NFTokenCreateOfferFlag {
    /// If enabled, indicates that the offer is a sell offer.
    /// Otherwise, it is a buy offer.
    TfSellOffer,
}

/// Transactions of the NFTokenMint type support additional values
/// in the Flags field. This enum represents those options.
///
/// See NFTokenMint flags:
/// `<https://xrpl.org/nftokenmint.html#nftokenmint-flags>`
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Display, AsRefStr)]
pub enum NFTokenMintFlag {
    /// Allow the issuer (or an entity authorized by the issuer) to
    /// destroy the minted NFToken. (The NFToken's owner can always do so.)
    TfBurnable,
    /// The minted NFToken can only be bought or sold for XRP.
    /// This can be desirable if the token has a transfer fee and the issuer
    /// does not want to receive fees in non-XRP currencies.
    TfOnlyXRP,
    /// Automatically create trust lines from the issuer to hold transfer
    /// fees received from transferring the minted NFToken.
    TfTrustline,
    /// The minted NFToken can be transferred to others. If this flag is not
    /// enabled, the token can still be transferred from or to the issuer.
    TfTransferable,
}

/// Transactions of the OfferCreate type support additional values
/// in the Flags field. This enum represents those options.
///
/// See OfferCreate flags:
/// `<https://xrpl.org/offercreate.html#offercreate-flags>`
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Display, AsRefStr)]
pub enum OfferCreateFlag {
    /// If enabled, the Offer does not consume Offers that exactly match it,
    /// and instead becomes an Offer object in the ledger.
    /// It still consumes Offers that cross it.
    TfPassive,
    /// Treat the Offer as an Immediate or Cancel order. The Offer never creates
    /// an Offer object in the ledger: it only trades as much as it can by
    /// consuming existing Offers at the time the transaction is processed. If no
    /// Offers match, it executes "successfully" without trading anything.
    /// In this case, the transaction still uses the result code tesSUCCESS.
    TfImmediateOrCancel,
    /// Treat the offer as a Fill or Kill order . The Offer never creates an Offer
    /// object in the ledger, and is canceled if it cannot be fully filled at the
    /// time of execution. By default, this means that the owner must receive the
    /// full TakerPays amount; if the tfSell flag is enabled, the owner must be
    /// able to spend the entire TakerGets amount instead.
    TfFillOrKill,
    /// Exchange the entire TakerGets amount, even if it means obtaining more than
    /// the TakerPays amount in exchange.
    TfSell,
}

/// Transactions of the Payment type support additional values
/// in the Flags field. This enum represents those options.
///
/// See Payment flags:
/// `<https://xrpl.org/payment.html#payment-flags>`
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Display, AsRefStr)]
pub enum PaymentFlag {
    /// Do not use the default path; only use paths included in the Paths field.
    /// This is intended to force the transaction to take arbitrage opportunities.
    /// Most clients do not need this.
    TfNoDirectRipple,
    /// If the specified Amount cannot be sent without spending more than SendMax,
    /// reduce the received amount instead of failing outright.
    /// See Partial Payments for more details.
    TfPartialPayment,
    /// Only take paths where all the conversions have an input:output ratio that
    /// is equal or better than the ratio of Amount:SendMax.
    /// See Limit Quality for details.
    TfLimitQuality,
}

/// Transactions of the PaymentChannelClaim type support additional values
/// in the Flags field. This enum represents those options.
///
/// See PaymentChannelClaim flags:
/// `<https://xrpl.org/paymentchannelclaim.html#paymentchannelclaim-flags>`
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Display, AsRefStr)]
pub enum PaymentChannelClaimFlag {
    /// Clear the channel's Expiration time. (Expiration is different from the
    /// channel's immutable CancelAfter time.) Only the source address of the
    /// payment channel can use this flag.
    TfRenew,
    /// Request to close the channel. Only the channel source and destination
    /// addresses can use this flag. This flag closes the channel immediately if
    /// it has no more XRP allocated to it after processing the current claim,
    /// or if the destination address uses it. If the source address uses this
    /// flag when the channel still holds XRP, this schedules the channel to close
    /// after SettleDelay seconds have passed. (Specifically, this sets the Expiration
    /// of the channel to the close time of the previous ledger plus the channel's
    /// SettleDelay time, unless the channel already has an earlier Expiration time.)
    /// If the destination address uses this flag when the channel still holds XRP,
    /// any XRP that remains after processing the claim is returned to the source address.
    TfClose,
}

/// Transactions of the TrustSet type support additional values
/// in the Flags field. This enum represents those options.
///
/// See TrustSet flags:
/// `<https://xrpl.org/trustset.html#trustset-flags>`
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Display, AsRefStr)]
pub enum TrustSetFlag {
    /// Authorize the other party to hold currency issued by this account.
    /// (No effect unless using the asfRequireAuth AccountSet flag.) Cannot be unset.
    TfSetAuth,
    /// Enable the No Ripple flag, which blocks rippling between two trust lines
    /// of the same currency if this flag is enabled on both.
    TfSetNoRipple,
    /// Disable the No Ripple flag, allowing rippling on this trust line.)
    TfClearNoRipple,
    /// Freeze the trust line.
    TfSetFreeze,
    /// Unfreeze the trust line.
    TfClearFreeze,
}

/// Pseudo-Transaction of the EnableAmendment type support additional values
/// in the Flags field. This enum represents those options.
///
/// See EnableAmendment flags:
/// `<https://xrpl.org/enableamendment.html#enableamendment-flags>`
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Display, AsRefStr)]
pub enum EnableAmendmentFlag {
    /// Support for this amendment increased to at least 80% of trusted
    /// validators starting with this ledger version.
    TfGotMajority,
    /// Support for this amendment decreased to less than 80% of trusted
    /// validators starting with this ledger version.
    TfLostMajority,
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
    RippleState,
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
        value: Option<Cow<'static, str>>,
        currency: Cow<'static, str>,
        issuer: Cow<'static, str>,
    },
    /// Specifies an amount in XRP.
    Xrp {
        value: Option<Cow<'static, str>>,
        currency: Cow<'static, str>,
    },
}

/// Enum containing the different Transaction types.
#[derive(Debug, Clone, Serialize, Deserialize, Display, PartialEq)]
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
    NFTokenAcceptOffer,
    NFTokenBurn,
    NFTokenCancelOffer,
    NFTokenCreateOffer,
    NFTokenMint,
    OfferCancel,
    OfferCreate,
    Payment,
    PaymentChannelClaim,
    PaymentChannelCreate,
    PaymentChannelFund,
    SetRegularKey,
    SignerListSet,
    TicketCreate,
    TrustSet,

    // Psuedo-Transaction types,
    EnableAmendment,
    SetFee,
    UNLModify,
}

/// Enum representing the options for the address role in
/// a NoRippleCheckRequest.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[serde(tag = "role")]
pub enum NoRippleCheckRole {
    Gateway,
    User,
}

/// There are three different modes, or sub-commands, of
/// the path_find command. Specify which one you want with
/// the subcommand parameter:
/// * create - Start sending pathfinding information
/// * close - Stop sending pathfinding information
/// * status - Info on the currently-open pathfinding request
///
/// See Path Find:
/// `<https://xrpl.org/path_find.html>`
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[serde(tag = "subcommand")]
pub enum PathFindSubcommand {
    Create,
    Close,
    Status,
}

/// Represents possible values of the streams query param
/// for subscribe.
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[serde(tag = "streams")]
pub enum StreamParameter {
    Consensus,
    Ledger,
    Manifests,
    PeerStatus,
    Transactions,
    TransactionsProposed,
    Server,
    Validations,
}

/// An arbitrary piece of data attached to a transaction. A
/// transaction can have multiple Memo objects as an array
/// in the Memos field.
///
/// Must contain one or more of `memo_data`, `memo_format`,
/// and `memo_type`.
///
/// See Memos Field:
/// `<https://xrpl.org/transaction-common-fields.html#memos-field>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct Memo<'a> {
    memo_data: Option<&'a str>,
    memo_format: Option<&'a str>,
    memo_type: Option<&'a str>,
}

/// A PathStep represents an individual step along a Path.
#[derive(Debug, Serialize, Deserialize)]
pub struct PathStep<'a> {
    account: Option<&'a str>,
    currency: Option<&'a str>,
    issuer: Option<&'a str>,
    r#type: Option<u8>,
    type_hex: Option<&'a str>,
}

/// One Signer in a multi-signature. A multi-signed transaction
/// can have an array of up to 8 Signers, each contributing a
/// signature, in the Signers field.
///
/// See Signers Field:
/// `<https://xrpl.org/transaction-common-fields.html#signers-field>`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct Signer<'a> {
    account: &'a str,
    txn_signature: &'a str,
    signing_pub_key: &'a str,
}

/// Returns a Currency as XRP for the currency, without a value.
pub fn default_xrp_currency() -> Currency {
    Currency::Xrp {
        value: None,
        currency: Borrowed("XRP"),
    }
}

/// For use with serde defaults.
fn default_true() -> Option<bool> {
    Some(true)
}

/// For use with serde defaults.
fn default_false() -> Option<bool> {
    Some(false)
}

/// For use with serde defaults.
fn default_limit_200() -> Option<u16> {
    Some(200)
}

/// For use with serde defaults.
fn default_limit_300() -> Option<u16> {
    Some(300)
}

/// For use with serde defaults.
fn default_fee_mult_max() -> Option<u32> {
    Some(10)
}

/// For use with serde defaults.
fn default_fee_div_max() -> Option<u32> {
    Some(1)
}

/// For use with serde defaults.
fn default_account_zero() -> &'static str {
    "rrrrrrrrrrrrrrrrrrrrrhoLvTp"
}

/// Allows creation of a Model object based on a JSON-like
/// dictionary of keys in the JSON format used by the binary
/// codec, or an actual JSON string representing the same data.
pub trait FromXRPL<T> {
    fn from_xrpl(value: T) -> Self;
}

impl Currency {
    fn get_value_as_u32(&self) -> u32 {
        match self {
            Currency::IssuedCurrency {
                value,
                currency: _,
                issuer: _,
            } => {
                let value_as_u32: u32 = value
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .parse()
                    .expect("Could not parse u32 from `value`");
                value_as_u32
            }
            Currency::Xrp { value, currency: _ } => {
                let value_as_u32: u32 = value
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .parse()
                    .expect("Could not parse u32 from `value`");
                value_as_u32
            }
        }
    }
    fn is_xrp(&self) -> bool {
        match self {
            Currency::IssuedCurrency {
                value: _,
                currency: _,
                issuer: _,
            } => false,
            Currency::Xrp {
                value: _,
                currency: _,
            } => true,
        }
    }
}

/// For use with serde defaults.
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
    fn fee() -> Self {
        RequestMethod::Fee
    }
    fn ledger_closed() -> Self {
        RequestMethod::LedgerClosed
    }
    fn ledger_current() -> Self {
        RequestMethod::LedgerCurrent
    }
    fn ledger_data() -> Self {
        RequestMethod::LedgerData
    }
    fn ledger_entry() -> Self {
        RequestMethod::LedgerEntry
    }
    fn ledger() -> Self {
        RequestMethod::Ledger
    }
    fn manifest() -> Self {
        RequestMethod::Manifest
    }
    fn no_ripple_check() -> Self {
        RequestMethod::NoRippleCheck
    }
    fn path_find() -> Self {
        RequestMethod::PathFind
    }
    fn ripple_path_find() -> Self {
        RequestMethod::RipplePathFind
    }
    fn ping() -> Self {
        RequestMethod::Ping
    }
    fn random() -> Self {
        RequestMethod::Random
    }
    fn server_info() -> Self {
        RequestMethod::ServerInfo
    }
    fn server_state() -> Self {
        RequestMethod::ServerState
    }
    fn submit() -> Self {
        RequestMethod::Submit
    }
    fn sign_for() -> Self {
        RequestMethod::SignFor
    }
    fn sign() -> Self {
        RequestMethod::Sign
    }
    fn submit_multisigned() -> Self {
        RequestMethod::SubmitMultisigned
    }
    fn subscribe() -> Self {
        RequestMethod::Subscribe
    }
    fn unsubscribe() -> Self {
        RequestMethod::Unsubscribe
    }
    fn transaction_entry() -> Self {
        RequestMethod::TransactionEntry
    }
    fn tx() -> Self {
        RequestMethod::Tx
    }
}

/// Standard functions for transactions.
pub trait Transaction {
    fn has_flag(&self, flag: &Flag) -> bool {
        let _txn_flag = flag;
        false
    }

    fn get_transaction_type(&self) -> TransactionType;
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct SignerEntry<'a> {
    account: &'a str,
    signer_weight: u16,
}

pub trait AccountSetError {
    fn _get_tick_size_error(&self) -> Result<(), AccountSetException>;
    fn _get_transfer_rate_error(&self) -> Result<(), AccountSetException>;
    fn _get_domain_error(&self) -> Result<(), AccountSetException>;
    fn _get_clear_flag_error(&self) -> Result<(), AccountSetException>;
    fn _get_nftoken_minter_error(&self) -> Result<(), AccountSetException>;
}

pub trait CheckCashError {
    fn _get_amount_and_deliver_min_error(&self) -> Result<(), CheckCashException>;
}

pub trait DepositPreauthError {
    fn _get_authorize_and_unauthorize_error(&self) -> Result<(), DepositPreauthException>;
}

pub trait EscrowCreateError {
    fn _get_finish_after_error(&self) -> Result<(), EscrowCreateException>;
}

pub trait EscrowFinishError {
    fn _get_condition_and_fulfillment_error(&self) -> Result<(), EscrowFinishException>;
}

pub trait NFTokenAcceptOfferError {
    fn _get_brokered_mode_error(&self) -> Result<(), NFTokenAcceptOfferException>;
    fn _get_nftoken_broker_fee_error(&self) -> Result<(), NFTokenAcceptOfferException>;
}

pub trait NFTokenCancelOfferError {
    fn _get_nftoken_offers_error(&self) -> Result<(), NFTokenCancelOfferException>;
}

pub trait NFTokenCreateOfferError {
    fn _get_amount_error(&self) -> Result<(), NFTokenCreateOfferException>;
    fn _get_destination_error(&self) -> Result<(), NFTokenCreateOfferException>;
    fn _get_owner_error(&self) -> Result<(), NFTokenCreateOfferException>;
}

pub trait NFTokenMintError {
    fn _get_issuer_error(&self) -> Result<(), NFTokenMintException>;
    fn _get_transfer_fee_error(&self) -> Result<(), NFTokenMintException>;
    fn _get_uri_error(&self) -> Result<(), NFTokenMintException>;
}

pub trait PaymentError {
    fn _get_xrp_transaction_error(&self) -> Result<(), PaymentException>;
    fn _get_partial_payment_error(&self) -> Result<(), PaymentException>;
    fn _get_exchange_error(&self) -> Result<(), PaymentException>;
}

pub trait SignerListSetError {
    fn _get_signer_entries_error(&self) -> Result<(), SignerListSetException>;
    fn _get_signer_quorum_error(&self) -> Result<(), SignerListSetException>;
}

pub trait UNLModifyError {
    fn _get_unl_modify_error(&self) -> Result<(), UNLModifyException>;
}

pub trait ChannelAuthorizeError {
    fn _get_field_error(&self) -> Result<(), ChannelAuthorizeException>;
}

pub trait LedgerEntryError {
    fn _get_field_error(&self) -> Result<(), LedgerEntryException>;
}

pub trait SignAndSubmitError {
    fn _get_field_error(&self) -> Result<(), SignAndSubmitException>;
    fn _get_key_type_error(&self) -> Result<(), SignAndSubmitException>;
}

pub trait SignForError {
    fn _get_field_error(&self) -> Result<(), SignForException>;
    fn _get_key_type_error(&self) -> Result<(), SignForException>;
}

pub trait SignError {
    fn _get_field_error(&self) -> Result<(), SignException>;
    fn _get_key_type_error(&self) -> Result<(), SignException>;
}

/// For use with serde defaults.
/// TODO Find a better way
impl TransactionType {
    fn account_delete() -> Self {
        TransactionType::AccountDelete
    }
    fn account_set() -> Self {
        TransactionType::AccountSet
    }
    fn check_cancel() -> Self {
        TransactionType::CheckCancel
    }
    fn check_cash() -> Self {
        TransactionType::CheckCash
    }
    fn check_create() -> Self {
        TransactionType::CheckCreate
    }
    fn deposit_preauth() -> Self {
        TransactionType::DepositPreauth
    }
    fn escrow_cancel() -> Self {
        TransactionType::EscrowCancel
    }
    fn escrow_create() -> Self {
        TransactionType::EscrowCreate
    }
    fn escrow_finish() -> Self {
        TransactionType::EscrowFinish
    }
    fn nftoken_accept_offer() -> Self {
        TransactionType::NFTokenAcceptOffer
    }
    fn nftoken_burn() -> Self {
        TransactionType::NFTokenBurn
    }
    fn nftoken_cancel_offer() -> Self {
        TransactionType::NFTokenCancelOffer
    }
    fn nftoken_create_offer() -> Self {
        TransactionType::NFTokenCreateOffer
    }
    fn nftoken_mint() -> Self {
        TransactionType::NFTokenMint
    }
    fn offer_cancel() -> Self {
        TransactionType::OfferCancel
    }
    fn offer_create() -> Self {
        TransactionType::OfferCreate
    }
    fn payment() -> Self {
        TransactionType::Payment
    }
    fn payment_channel_claim() -> Self {
        TransactionType::PaymentChannelClaim
    }
    fn payment_channel_create() -> Self {
        TransactionType::PaymentChannelCreate
    }
    fn payment_channel_fund() -> Self {
        TransactionType::PaymentChannelFund
    }
    fn set_regular_key() -> Self {
        TransactionType::SetRegularKey
    }
    fn signer_list_set() -> Self {
        TransactionType::SignerListSet
    }
    fn ticket_create() -> Self {
        TransactionType::TicketCreate
    }
    fn trust_set() -> Self {
        TransactionType::TrustSet
    }
    fn enable_amendment() -> Self {
        TransactionType::EnableAmendment
    }
    fn set_fee() -> Self {
        TransactionType::SetFee
    }
    fn unl_modify() -> Self {
        TransactionType::UNLModify
    }
}
