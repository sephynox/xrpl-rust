//! Request models.

use crate::constants::CryptoAlgorithm;
use crate::models::*;
use alloc::{string::ToString, vec::Vec};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{
    exceptions::{SignAndSubmitException, XRPLModelException, XRPLRequestException},
    request_fields::*,
};

/// This request returns information about an account's Payment
/// Channels. This includes only channels where the specified
/// account is the channel's source, not the destination.
/// (A channel's "source" and "owner" are the same.) All
/// information retrieved is relative to a particular version
/// of the ledger.
///
/// See Account Channels:
/// `<https://xrpl.org/account_channels.html>`
///
/// # Examples
///
/// ## Basic usage
///
/// ```
/// use xrpl::models::AccountChannels;
///
/// let json = r#"{"account":"rH6ZiHU1PGamME2LvVTxrgvfjQpppWKGmr","marker":12345678,"command":"account_channels"}"#.to_string();
/// let model: AccountChannels = serde_json::from_str(&json).expect("");
/// let revert: Option<String> = match serde_json::to_string(&model) {
///     Ok(model) => Some(model),
///     Err(_) => None,
/// };
///
/// assert_eq!(revert, Some(json));
/// ```
#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountChannels<'a> {
    /// The unique identifier of an account, typically the
    /// account's Address. The request returns channels where
    /// this account is the channel's owner/source.
    pub account: &'a str,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// A 20-byte hex string for the ledger version to use.
    pub ledger_hash: Option<&'a str>,
    /// The ledger index of the ledger to use, or a shortcut
    /// string to choose a ledger automatically.
    pub ledger_index: Option<&'a str>,
    /// Limit the number of transactions to retrieve. Cannot
    /// be less than 10 or more than 400. The default is 200.
    pub limit: Option<u16>,
    /// The unique identifier of an account, typically the
    /// account's Address. If provided, filter results to
    /// payment channels whose destination is this account.
    pub destination_account: Option<&'a str>,
    /// Value from a previous paginated response.
    /// Resume retrieving data where that response left off.
    pub marker: Option<u32>,
    /// The request method.
    #[serde(default = "RequestMethod::account_channels")]
    pub command: RequestMethod,
}

impl Default for AccountChannels<'static> {
    fn default() -> Self {
        AccountChannels {
            account: "",
            id: None,
            ledger_hash: None,
            ledger_index: None,
            limit: None,
            destination_account: None,
            marker: None,
            command: RequestMethod::AccountChannels,
        }
    }
}

impl Model for AccountChannels<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `AccountChannels` to json.")
    }
}

/// This request retrieves a list of currencies that an account
/// can send or receive, based on its trust lines. This is not
/// a thoroughly confirmed list, but it can be used to populate
/// user interfaces.
///
/// See Account Currencies:
/// `<https://xrpl.org/account_currencies.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountCurrencies<'a> {
    /// A unique identifier for the account, most commonly
    /// the account's Address.
    pub account: &'a str,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// A 20-byte hex string for the ledger version to use.
    pub ledger_hash: Option<&'a str>,
    /// The ledger index of the ledger to use, or a shortcut
    /// string to choose a ledger automatically.
    pub ledger_index: Option<&'a str>,
    /// If true, then the account field only accepts a public
    /// key or XRP Ledger address. Otherwise, account can be
    /// a secret or passphrase (not recommended).
    /// The default is false.
    #[serde(default = "default_false")]
    pub strict: Option<bool>,
    /// The request method.
    #[serde(default = "RequestMethod::account_currencies")]
    pub command: RequestMethod,
}

impl Default for AccountCurrencies<'static> {
    fn default() -> Self {
        AccountCurrencies {
            account: "",
            id: None,
            ledger_hash: None,
            ledger_index: None,
            strict: None,
            command: RequestMethod::AccountCurrencies,
        }
    }
}

impl Model for AccountCurrencies<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `AccountCurrencies` to json.")
    }
}

/// This request retrieves information about an account, its
/// activity, and its XRP balance. All information retrieved
/// is relative to a particular version of the ledger.
///
/// See Account Info:
/// `<https://xrpl.org/account_info.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo<'a> {
    /// A unique identifier for the account, most commonly the
    /// account's Address.
    pub account: &'a str,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// A 20-byte hex string for the ledger version to use.
    pub ledger_hash: Option<&'a str>,
    /// The ledger index of the ledger to use, or a shortcut
    /// string to choose a ledger automatically.
    pub ledger_index: Option<&'a str>,
    /// If true, then the account field only accepts a public
    /// key or XRP Ledger address. Otherwise, account can be
    /// a secret or passphrase (not recommended).
    /// The default is false.
    #[serde(default = "default_false")]
    pub strict: Option<bool>,
    /// If true, and the FeeEscalation amendment is enabled,
    /// also returns stats about queued transactions associated
    /// with this account. Can only be used when querying for the
    /// data from the current open ledger. New in: rippled 0.33.0
    /// Not available from servers in Reporting Mode.
    #[serde(default = "default_false")]
    pub queue: Option<bool>,
    /// If true, and the MultiSign amendment is enabled, also
    /// returns any SignerList objects associated with this account.
    #[serde(default = "default_false")]
    pub signer_lists: Option<bool>,
    /// The request method.
    #[serde(default = "RequestMethod::account_info")]
    pub command: RequestMethod,
}

impl Default for AccountInfo<'static> {
    fn default() -> Self {
        AccountInfo {
            account: "",
            id: None,
            ledger_hash: None,
            ledger_index: None,
            strict: None,
            queue: None,
            signer_lists: None,
            command: RequestMethod::AccountInfo,
        }
    }
}

impl Model for AccountInfo<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `AccountInfo` to json.")
    }
}

/// This request returns information about an account's trust
/// lines, including balances in all non-XRP currencies and
/// assets. All information retrieved is relative to a particular
/// version of the ledger.
///
/// See Account Lines:
/// `<https://xrpl.org/account_lines.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountLines<'a> {
    /// A unique identifier for the account, most commonly the
    /// account's Address.
    pub account: &'a str,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// A 20-byte hex string for the ledger version to use.
    pub ledger_hash: Option<&'a str>,
    /// The ledger index of the ledger to use, or a shortcut
    /// string to choose a ledger automatically.
    pub ledger_index: Option<&'a str>,
    /// Limit the number of trust lines to retrieve. The server
    /// is not required to honor this value. Must be within the
    /// inclusive range 10 to 400.
    pub limit: Option<u16>,
    /// The Address of a second account. If provided, show only
    /// lines of trust connecting the two accounts.
    pub peer: Option<&'a str>,
    /// Value from a previous paginated response. Resume retrieving
    /// data where that response left off.
    pub marker: Option<u32>,
    /// The request method.
    #[serde(default = "RequestMethod::account_lines")]
    pub command: RequestMethod,
}

impl Default for AccountLines<'static> {
    fn default() -> Self {
        AccountLines {
            account: "",
            id: None,
            ledger_hash: None,
            ledger_index: None,
            limit: None,
            peer: None,
            marker: None,
            command: RequestMethod::AccountLines,
        }
    }
}

impl Model for AccountLines<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `AccountLines` to json.")
    }
}

/// This method retrieves all of the NFTs currently owned
/// by the specified account.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountNfts<'a> {
    /// The unique identifier of an account, typically the
    /// account's Address. The request returns a list of
    /// NFTs owned by this account.
    pub account: &'a str,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// Limit the number of token pages to retrieve. Each page
    /// can contain up to 32 NFTs. The limit value cannot be
    /// lower than 20 or more than 400. The default is 100.
    pub limit: Option<u32>,
    /// Value from a previous paginated response. Resume
    /// retrieving data where that response left off.
    pub marker: Option<u32>,
    /// The request method.
    #[serde(default = "RequestMethod::account_nfts")]
    pub command: RequestMethod,
}

impl Default for AccountNfts<'static> {
    fn default() -> Self {
        AccountNfts {
            account: "",
            id: None,
            limit: None,
            marker: None,
            command: RequestMethod::AccountNfts,
        }
    }
}

impl Model for AccountNfts<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `AccountNfts` to json.")
    }
}

/// This request returns the raw ledger format for all objects
/// owned by an account. For a higher-level view of an account's
/// trust lines and balances, see AccountLines Request instead.
///
/// See Account Objects:
/// `<https://xrpl.org/account_objects.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountObjects<'a> {
    /// A unique identifier for the account, most commonly the
    /// account's address.
    pub account: &'a str,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// A 20-byte hex string for the ledger version to use.
    pub ledger_hash: Option<&'a str>,
    /// The ledger index of the ledger to use, or a shortcut
    /// string to choose a ledger automatically.
    pub ledger_index: Option<&'a str>,
    /// If included, filter results to include only this type
    /// of ledger object. The valid types are: check, deposit_preauth,
    /// escrow, offer, payment_channel, signer_list, ticket,
    /// and state (trust line).
    pub r#type: Option<AccountObjectType>,
    /// If true, the response only includes objects that would block
    /// this account from being deleted. The default is false.
    #[serde(default = "default_false")]
    pub deletion_blockers_only: Option<bool>,
    /// The maximum number of objects to include in the results.
    /// Must be within the inclusive range 10 to 400 on non-admin
    /// connections. The default is 200.
    pub limit: Option<u16>,
    /// Value from a previous paginated response. Resume retrieving
    /// data where that response left off.
    pub marker: Option<u32>,
    /// The request method.
    #[serde(default = "RequestMethod::account_objects")]
    pub command: RequestMethod,
}

impl Default for AccountObjects<'static> {
    fn default() -> Self {
        AccountObjects {
            account: "",
            id: None,
            ledger_hash: None,
            ledger_index: None,
            r#type: None,
            deletion_blockers_only: None,
            limit: None,
            marker: None,
            command: RequestMethod::AccountObjects,
        }
    }
}

impl Model for AccountObjects<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `AccountObjects` to json.")
    }
}

/// This request retrieves a list of offers made by a given account
/// that are outstanding as of a particular ledger version.
///
/// See Account Offers:
/// `<https://xrpl.org/account_offers.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountOffers<'a> {
    /// A unique identifier for the account, most commonly the
    /// account's Address.
    pub account: &'a str,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// A 20-byte hex string identifying the ledger version to use.
    pub ledger_hash: Option<&'a str>,
    /// The ledger index of the ledger to use, or "current",
    /// "closed", or "validated" to select a ledger dynamically.
    pub ledger_index: Option<&'a str>,
    /// Limit the number of transactions to retrieve. The server is
    /// not required to honor this value. Must be within the inclusive
    /// range 10 to 400.
    pub limit: Option<u16>,
    /// If true, then the account field only accepts a public key or
    /// XRP Ledger address. Otherwise, account can be a secret or
    /// passphrase (not recommended). The default is false.
    #[serde(default = "default_false")]
    pub strict: Option<bool>,
    /// Value from a previous paginated response. Resume retrieving
    /// data where that response left off.
    pub marker: Option<u32>,
    /// The request method.
    #[serde(default = "RequestMethod::account_offers")]
    pub command: RequestMethod,
}

impl Default for AccountOffers<'static> {
    fn default() -> Self {
        AccountOffers {
            account: "",
            id: None,
            ledger_hash: None,
            ledger_index: None,
            limit: None,
            strict: None,
            marker: None,
            command: RequestMethod::AccountOffers,
        }
    }
}

impl Model for AccountOffers<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `AccountOffers` to json.")
    }
}

/// This request retrieves from the ledger a list of
/// transactions that involved the specified account.
///
/// See Account Tx:
/// `<https://xrpl.org/account_tx.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountTx<'a> {
    /// A unique identifier for the account, most commonly the
    /// account's address.
    pub account: &'a str,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// Use to look for transactions from a single ledger only.
    pub ledger_hash: Option<&'a str>,
    /// Use to look for transactions from a single ledger only.
    pub ledger_index: Option<&'a str>,
    /// Defaults to false. If set to true, returns transactions
    /// as hex strings instead of JSON.
    #[serde(default = "default_false")]
    pub binary: Option<bool>,
    /// Defaults to false. If set to true, returns values indexed
    /// with the oldest ledger first. Otherwise, the results are
    /// indexed with the newest ledger first.
    /// (Each page of results may not be internally ordered, but
    /// the pages are overall ordered.)
    #[serde(default = "default_false")]
    pub forward: Option<bool>,
    /// Use to specify the earliest ledger to include transactions
    /// from. A value of -1 instructs the server to use the earliest
    /// validated ledger version available.
    pub ledger_index_min: Option<u32>,
    /// Use to specify the most recent ledger to include transactions
    /// from. A value of -1 instructs the server to use the most
    /// recent validated ledger version available.
    pub ledger_index_max: Option<u32>,
    /// Default varies. Limit the number of transactions to retrieve.
    /// The server is not required to honor this value.
    pub limit: Option<u16>,
    /// Value from a previous paginated response. Resume retrieving
    /// data where that response left off. This value is stable even
    /// if there is a change in the server's range of available
    /// ledgers.
    pub marker: Option<u32>,
    /// The request method.
    #[serde(default = "RequestMethod::account_tx")]
    pub command: RequestMethod,
}

impl Default for AccountTx<'static> {
    fn default() -> Self {
        AccountTx {
            account: "",
            id: None,
            ledger_hash: None,
            ledger_index: None,
            binary: None,
            forward: None,
            ledger_index_min: None,
            ledger_index_max: None,
            limit: None,
            marker: None,
            command: RequestMethod::AccountTx,
        }
    }
}

impl Model for AccountTx<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `AccountTx` to json.")
    }
}

/// The book_offers method retrieves a list of offers, also known
/// as the order book, between two currencies.
///
/// See Book Offers:
/// `<https://xrpl.org/book_offers.html#book_offers>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct BookOffers<'a> {
    /// Specification of which currency the account taking
    /// the offer would receive, as an object with currency
    /// and issuer fields (omit issuer for XRP),
    /// like currency amounts.
    pub taker_gets: Currency,
    /// Specification of which currency the account taking
    /// the offer would pay, as an object with currency and
    /// issuer fields (omit issuer for XRP),
    /// like currency amounts.
    pub taker_pays: Currency,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// A 20-byte hex string for the ledger version to use.
    pub ledger_hash: Option<&'a str>,
    /// The ledger index of the ledger to use, or a shortcut
    /// string to choose a ledger automatically.
    pub ledger_index: Option<&'a str>,
    /// If provided, the server does not provide more than
    /// this many offers in the results. The total number of
    /// results returned may be fewer than the limit,
    /// because the server omits unfunded offers.
    pub limit: Option<u16>,
    /// The Address of an account to use as a perspective.
    /// Unfunded offers placed by this account are always
    /// included in the response. (You can use this to look
    /// up your own orders to cancel them.)
    pub taker: Option<&'a str>,
    /// The request method.
    #[serde(default = "RequestMethod::book_offers")]
    pub command: RequestMethod,
}

impl Default for BookOffers<'static> {
    fn default() -> Self {
        BookOffers {
            taker_gets: Currency::Xrp {
                value: None,
                currency: Default::default(),
            },
            taker_pays: Currency::Xrp {
                value: None,
                currency: Default::default(),
            },
            id: None,
            ledger_hash: None,
            ledger_index: None,
            limit: None,
            taker: None,
            command: RequestMethod::BookOffers,
        }
    }
}

impl Model for BookOffers<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `BookOffers` to json.")
    }
}

/// The channel_authorize method creates a signature that can  be
/// used to redeem a specific amount of XRP from a payment channel.
///
/// Warning: Do not send secret keys to untrusted servers or
/// through unsecured network connections. (This includes the
/// secret, seed, seed_hex, or passphrase fields of this request.)
/// You should only use this method on a secure, encrypted network
/// connection to a server you run or fully trust with your funds.
/// Otherwise, eavesdroppers could use your secret key to sign
/// claims and take all the money from this payment channel and
/// anything else using the same key pair.
///
/// See Set Up Secure Signing:
/// `<https://xrpl.org/set-up-secure-signing.html>`
///
/// See Channel Authorize:
/// `<https://xrpl.org/channel_authorize.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelAuthorize<'a> {
    /// The unique ID of the payment channel to use.
    pub channel_id: &'a str,
    /// Cumulative amount of XRP, in drops, to authorize.
    /// If the destination has already received a lesser amount
    /// of XRP from this channel, the signature created by this
    /// method can be redeemed for the difference.
    pub amount: &'a str,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// The secret key to use to sign the claim. This must be
    /// the same key pair as the public key specified in the
    /// channel. Cannot be used with seed, seed_hex, or passphrase.
    pub secret: Option<&'a str>,
    /// The secret seed to use to sign the claim. This must be
    /// the same key pair as the public key specified in the channel.
    /// Must be in the XRP Ledger's base58 format. If provided,
    /// you must also specify the key_type. Cannot be used with
    /// secret, seed_hex, or passphrase.
    pub seed: Option<&'a str>,
    /// The secret seed to use to sign the claim. This must be the
    /// same key pair as the public key specified in the channel.
    /// Must be in hexadecimal format. If provided, you must also
    /// specify the key_type. Cannot be used with secret, seed,
    /// or passphrase.
    pub seed_hex: Option<&'a str>,
    /// A string passphrase to use to sign the claim. This must be
    /// the same key pair as the public key specified in the channel.
    /// The key derived from this passphrase must match the public
    /// key specified in the channel. If provided, you must also
    /// specify the key_type. Cannot be used with secret, seed,
    /// or seed_hex.
    pub passphrase: Option<&'a str>,
    /// The signing algorithm of the cryptographic key pair provided.
    /// Valid types are secp256k1 or ed25519. The default is secp256k1.
    pub key_type: Option<CryptoAlgorithm>,
    /// The request method.
    #[serde(default = "RequestMethod::channel_authorize")]
    pub command: RequestMethod,
}

impl Default for ChannelAuthorize<'static> {
    fn default() -> Self {
        ChannelAuthorize {
            channel_id: "",
            amount: "",
            id: None,
            secret: None,
            seed: None,
            seed_hex: None,
            passphrase: None,
            key_type: None,
            command: RequestMethod::ChannelAuthorize,
        }
    }
}

impl Model for ChannelAuthorize<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `ChannelAuthorize` to json.")
    }

    fn get_errors(&self) -> Result<(), exceptions::XRPLModelException> {
        match self._get_field_error() {
            Err(error) => Err(XRPLModelException::XRPLRequestError(
                XRPLRequestException::ChannelAuthorizeError(error),
            )),
            Ok(_no_error) => Ok(()),
        }
    }
}

impl ChannelAuthorizeError for ChannelAuthorize<'static> {
    fn _get_field_error(&self) -> Result<(), ChannelAuthorizeException> {
        let mut signing_methods = Vec::new();
        for method in [self.secret, self.seed, self.seed_hex, self.passphrase] {
            if method.is_some() {
                signing_methods.push(method)
            }
        }
        match signing_methods.len() != 1 {
            true => Err(ChannelAuthorizeException::InvalidMustSetExactlyOneOf {
                fields: "`secret`, `seed`, `seed_hex`, `passphrase`".to_string(),
            }),
            false => Ok(()),
        }
    }
}

/// The channel_verify method checks the validity of a signature
/// that can be used to redeem a specific amount of XRP from a
/// payment channel.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelVerify<'a> {
    /// The Channel ID of the channel that provides the XRP.
    /// This is a 64-character hexadecimal string.
    pub channel_id: &'a str,
    /// The amount of XRP, in drops, the provided signature authorizes.
    pub amount: &'a str,
    /// The public key of the channel and the key pair that was used to
    /// create the signature, in hexadecimal or the XRP Ledger's
    /// base58 format.
    pub public_key: &'a str,
    /// The signature to verify, in hexadecimal.
    pub signature: &'a str,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// The request method.
    #[serde(default = "RequestMethod::channel_verify")]
    pub command: RequestMethod,
}

impl Default for ChannelVerify<'static> {
    fn default() -> Self {
        ChannelVerify {
            channel_id: "",
            amount: "",
            public_key: "",
            signature: "",
            id: None,
            command: RequestMethod::ChannelVerify,
        }
    }
}

impl Model for ChannelVerify<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `ChannelVerify` to json.")
    }
}

/// The deposit_authorized command indicates whether one account
/// is authorized to send payments directly to another.
///
/// See Deposit Authorization:
/// `<https://xrpl.org/depositauth.html#deposit-authorization>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositAuthorized<'a> {
    /// The sender of a possible payment.
    pub source_account: &'a str,
    /// The recipient of a possible payment.
    pub destination_account: &'a str,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// A 20-byte hex string for the ledger version to use.
    pub ledger_hash: Option<&'a str>,
    /// The ledger index of the ledger to use, or a shortcut
    /// string to choose a ledger automatically.
    pub ledger_index: Option<&'a str>,
    /// The request method.
    #[serde(default = "RequestMethod::deposit_authorization")]
    pub command: RequestMethod,
}

impl Default for DepositAuthorized<'static> {
    fn default() -> Self {
        DepositAuthorized {
            source_account: "",
            destination_account: "",
            id: None,
            ledger_hash: None,
            ledger_index: None,
            command: RequestMethod::DepositAuthorized,
        }
    }
}

impl Model for DepositAuthorized<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `DepositAuthorized` to json.")
    }
}

/// The fee command reports the current state of the open-ledger
/// requirements for the transaction cost. This requires the
/// FeeEscalation amendment to be enabled. This is a public
/// command available to unprivileged users.
///
/// See Fee:
/// `<https://xrpl.org/fee.html#fee>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Fee<'a> {
    /// The unique request id.
    pub id: Option<&'a str>,
    /// The request method.
    #[serde(default = "RequestMethod::fee")]
    pub command: RequestMethod,
}

impl Default for Fee<'static> {
    fn default() -> Self {
        Fee {
            id: None,
            command: RequestMethod::Fee,
        }
    }
}

impl Model for Fee<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `Fee` to json.")
    }
}

/// This request calculates the total balances issued by a
/// given account, optionally excluding amounts held by
/// operational addresses.
///
/// See Gateway Balances:
/// `<https://xrpl.org/gateway_balances.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayBalances<'a> {
    /// The Address to check. This should be the issuing address.
    pub account: &'a str,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// If true, only accept an address or public key for the
    /// account parameter. Defaults to false.
    #[serde(default = "default_false")]
    pub strict: Option<bool>,
    /// A 20-byte hex string for the ledger version to use.
    pub ledger_hash: Option<&'a str>,
    /// The ledger index of the ledger version to use, or a
    /// shortcut string to choose a ledger automatically.
    pub ledger_index: Option<&'a str>,
    /// An operational address to exclude from the balances
    /// issued, or an array of such addresses.
    pub hotwallet: Option<Vec<&'a str>>,
    /// The request method.
    #[serde(default = "RequestMethod::deposit_authorization")]
    pub command: RequestMethod,
}

impl Default for GatewayBalances<'static> {
    fn default() -> Self {
        GatewayBalances {
            account: "",
            id: None,
            strict: None,
            ledger_hash: None,
            ledger_index: None,
            hotwallet: None,
            command: RequestMethod::GatewayBalances,
        }
    }
}

impl Model for GatewayBalances<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `GatewayBalances` to json.")
    }
}

/// The ledger_closed method returns the unique identifiers of
/// the most recently closed ledger. (This ledger is not
/// necessarily validated and immutable yet.)
///
/// See Ledger Closed:
/// `<https://xrpl.org/ledger_closed.html#ledger_closed>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct LedgerClosed<'a> {
    /// The unique request id.
    pub id: Option<&'a str>,
    /// The request method.
    #[serde(default = "RequestMethod::ledger_closed")]
    pub command: RequestMethod,
}

impl Default for LedgerClosed<'static> {
    fn default() -> Self {
        LedgerClosed {
            id: None,
            command: RequestMethod::LedgerClosed,
        }
    }
}

impl Model for LedgerClosed<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `LedgerClosed` to json.")
    }
}

/// The ledger_closed method returns the unique identifiers of
/// the most recently closed ledger. (This ledger is not
/// necessarily validated and immutable yet.)
///
/// See Ledger Closed:
/// `<https://xrpl.org/ledger_closed.html#ledger_closed>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct LedgerCurrent<'a> {
    /// The unique request id.
    pub id: Option<&'a str>,
    /// The request method.
    #[serde(default = "RequestMethod::ledger_current")]
    pub command: RequestMethod,
}

impl Default for LedgerCurrent<'static> {
    fn default() -> Self {
        LedgerCurrent {
            id: None,
            command: RequestMethod::LedgerCurrent,
        }
    }
}

impl Model for LedgerCurrent<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `LedgerCurrent` to json.")
    }
}

/// The ledger_data method retrieves contents of the specified
/// ledger. You can iterate through several calls to retrieve
/// the entire contents of a single ledger version.
///
/// See Ledger Data:
/// `<https://xrpl.org/ledger_data.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct LedgerData<'a> {
    /// The unique request id.
    pub id: Option<&'a str>,
    /// A 20-byte hex string for the ledger version to use.
    pub ledger_hash: Option<&'a str>,
    /// The ledger index of the ledger to use, or a shortcut
    /// string to choose a ledger automatically.
    pub ledger_index: Option<&'a str>,
    /// If set to true, return ledger objects as hashed hex
    /// strings instead of JSON.
    #[serde(default = "default_false")]
    pub binary: Option<bool>,
    /// Limit the number of ledger objects to retrieve.
    /// The server is not required to honor this value.
    pub limit: Option<u16>,
    /// Value from a previous paginated response.
    /// Resume retrieving data where that response left off.
    pub marker: Option<u32>,
    /// The request method.
    #[serde(default = "RequestMethod::ledger_data")]
    pub command: RequestMethod,
}

impl Default for LedgerData<'static> {
    fn default() -> Self {
        LedgerData {
            id: None,
            ledger_hash: None,
            ledger_index: None,
            binary: None,
            limit: None,
            marker: None,
            command: RequestMethod::LedgerData,
        }
    }
}

impl Model for LedgerData<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `LedgerData` to json.")
    }
}

/// The ledger_entry method returns a single ledger object
/// from the XRP Ledger in its raw format. See ledger formats
/// for information on the different types of objects you can
/// retrieve.
///
/// See Ledger Formats:
/// `<https://xrpl.org/ledger-data-formats.html#ledger-data-formats>`
///
/// See Ledger Entry:
/// `<https://xrpl.org/ledger_entry.html#ledger_entry>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct LedgerEntry<'a> {
    /// The unique request id.
    pub id: Option<&'a str>,
    pub index: Option<&'a str>,
    pub account_root: Option<&'a str>,
    pub check: Option<&'a str>,
    pub payment_channel: Option<&'a str>,
    pub deposit_preauth: Option<DepositPreauth<'a>>,
    pub directory: Option<DirectoryFields<'a>>,
    pub escrow: Option<EscrowFields<'a>>,
    pub offer: Option<OfferFields<'a>>,
    pub ripple_state: Option<RippleStateFields<'a>>,
    pub ticket: Option<TicketFields<'a>>,
    /// If true, return the requested ledger object's contents as a
    /// hex string in the XRP Ledger's binary format. Otherwise, return
    /// data in JSON format. The default is false.
    #[serde(default = "default_false")]
    pub binary: Option<bool>,
    /// A 20-byte hex string for the ledger version to use.
    pub ledger_hash: Option<&'a str>,
    /// The ledger index of the ledger to use, or a shortcut string
    /// (e.g. "validated" or "closed" or "current") to choose a ledger
    /// automatically.
    pub ledger_index: Option<&'a str>,
    /// The request method.
    #[serde(default = "RequestMethod::ledger_entry")]
    pub command: RequestMethod,
}

impl Default for LedgerEntry<'static> {
    fn default() -> Self {
        LedgerEntry {
            id: None,
            index: None,
            account_root: None,
            check: None,
            payment_channel: None,
            deposit_preauth: None,
            directory: None,
            escrow: None,
            offer: None,
            ripple_state: None,
            ticket: None,
            binary: None,
            ledger_hash: None,
            ledger_index: None,
            command: RequestMethod::LedgerEntry,
        }
    }
}

impl Model for LedgerEntry<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `LedgerEntry` to json.")
    }

    fn get_errors(&self) -> Result<(), exceptions::XRPLModelException> {
        match self._get_field_error() {
            Err(error) => Err(XRPLModelException::XRPLRequestError(
                XRPLRequestException::LedgerEntryError(error),
            )),
            Ok(_no_error) => Ok(()),
        }
    }
}

impl LedgerEntryError for LedgerEntry<'static> {
    fn _get_field_error(&self) -> Result<(), LedgerEntryException> {
        let mut signing_methods: u32 = 0;
        for method in [self.index, self.account_root, self.check] {
            if method.is_some() {
                signing_methods += 1
            }
        }
        if self.directory.is_some() {
            signing_methods += 1
        }
        if self.offer.is_some() {
            signing_methods += 1
        }
        if self.ripple_state.is_some() {
            signing_methods += 1
        }
        if self.escrow.is_some() {
            signing_methods += 1
        }
        if self.payment_channel.is_some() {
            signing_methods += 1
        }
        if self.deposit_preauth.is_some() {
            signing_methods += 1
        }
        if self.ticket.is_some() {
            signing_methods += 1
        }
        match signing_methods != 1 {
            true => Err(LedgerEntryException::InvalidMustSetExactlyOneOf { fields: "`index`, `account_root`, `check`, `directory`, `offer`, `ripple_state`, `escrow`, `payment_channel`, `deposit_preauth`, `ticket`".to_string() }),
            false => Ok(()),
        }
    }
}

/// Retrieve information about the public ledger.
///
/// See Ledger Data:
/// `<https://xrpl.org/ledger.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Ledger<'a> {
    /// The unique request id.
    pub id: Option<&'a str>,
    /// A 20-byte hex string for the ledger version to use.
    pub ledger_hash: Option<&'a str>,
    /// The ledger index of the ledger to use, or a shortcut
    /// string to choose a ledger automatically.
    pub ledger_index: Option<&'a str>,
    /// Admin required. If true, return full information on
    /// the entire ledger. Ignored if you did not specify a
    /// ledger version. Defaults to false. (Equivalent to
    /// enabling transactions, accounts, and expand.)
    /// Caution: This is a very large amount of data -- on
    /// the order of several hundred megabytes!
    #[serde(default = "default_false")]
    pub full: Option<bool>,
    /// Admin required. If true, return information on accounts
    /// in the ledger. Ignored if you did not specify a ledger
    /// version. Defaults to false. Caution: This returns a very
    /// large amount of data!
    #[serde(default = "default_false")]
    pub accounts: Option<bool>,
    /// If true, return information on transactions in the
    /// specified ledger version. Defaults to false. Ignored if
    /// you did not specify a ledger version.
    #[serde(default = "default_false")]
    pub transactions: Option<bool>,
    /// Provide full JSON-formatted information for
    /// transaction/account information instead of only hashes.
    /// Defaults to false. Ignored unless you request transactions,
    /// accounts, or both.
    #[serde(default = "default_false")]
    pub expand: Option<bool>,
    /// If true, include owner_funds field in the metadata of
    /// OfferCreate transactions in the response. Defaults to
    /// false. Ignored unless transactions are included and
    /// expand is true.
    #[serde(default = "default_false")]
    pub owner_funds: Option<bool>,
    /// If true, and transactions and expand are both also true,
    /// return transaction information in binary format
    /// (hexadecimal string) instead of JSON format.
    #[serde(default = "default_false")]
    pub binary: Option<bool>,
    /// If true, and the command is requesting the current ledger,
    /// includes an array of queued transactions in the results.
    #[serde(default = "default_false")]
    pub queue: Option<bool>,
    /// The request method.
    #[serde(default = "RequestMethod::ledger")]
    pub command: RequestMethod,
}

impl Default for Ledger<'static> {
    fn default() -> Self {
        Ledger {
            id: None,
            ledger_hash: None,
            ledger_index: None,
            full: None,
            accounts: None,
            transactions: None,
            expand: None,
            owner_funds: None,
            binary: None,
            queue: None,
            command: RequestMethod::Ledger,
        }
    }
}

impl Model for Ledger<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `Ledger` to json.")
    }
}

/// The manifest method reports the current "manifest"
/// information for a given validator public key. The
/// "manifest" is the public portion of that validator's
/// configured token.
///
/// See Manifest:
/// `<https://xrpl.org/manifest.html#manifest>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest<'a> {
    /// The base58-encoded public key of the validator
    /// to look up. This can be the master public key or
    /// ephemeral public key.
    pub public_key: &'a str,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// The request method.
    #[serde(default = "RequestMethod::manifest")]
    pub command: RequestMethod,
}

impl Default for Manifest<'static> {
    fn default() -> Self {
        Manifest {
            public_key: "",
            id: None,
            command: RequestMethod::Manifest,
        }
    }
}

impl Model for Manifest<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `Manifest` to json.")
    }
}

/// This method retrieves all of buy offers for the specified NFToken.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct NftBuyOffers<'a> {
    /// The unique identifier of a NFToken object.
    pub nft_id: &'a str,
    /// A 20-byte hex string for the ledger version to use.
    pub ledger_hash: Option<&'a str>,
    /// The ledger index of the ledger to use, or a shortcut
    /// string to choose a ledger automatically.
    pub ledger_index: Option<&'a str>,
    /// Limit the number of NFT buy offers to retrieve.
    /// This value cannot be lower than 50 or more than 500.
    /// The default is 250.
    pub limit: Option<u16>,
    /// Value from a previous paginated response.
    /// Resume retrieving data where that response left off.
    pub marker: Option<u32>,
    /// The request method.
    #[serde(default = "RequestMethod::nft_buy_offers")]
    pub command: RequestMethod,
}

impl Default for NftBuyOffers<'static> {
    fn default() -> Self {
        NftBuyOffers {
            nft_id: "",
            ledger_hash: None,
            ledger_index: None,
            limit: None,
            marker: None,
            command: RequestMethod::NftBuyOffers,
        }
    }
}

impl Model for NftBuyOffers<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `NftBuyOffers` to json.")
    }
}

/// This method retrieves all of sell offers for the specified NFToken.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct NftSellOffers<'a> {
    /// The unique identifier of a NFToken object.
    pub nft_id: &'a str,
    /// The request method.
    #[serde(default = "RequestMethod::nft_sell_offers")]
    pub command: RequestMethod,
}

impl Default for NftSellOffers<'static> {
    fn default() -> Self {
        NftSellOffers {
            nft_id: "",
            command: RequestMethod::NftSellOffers,
        }
    }
}

impl Model for NftSellOffers<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `NftSellOffers` to json.")
    }
}

/// This request provides a quick way to check the status of
/// the Default Ripple field for an account and the No Ripple
/// flag of its trust lines, compared with the recommended
/// settings.
///
/// See No Ripple Check:
/// `<https://xrpl.org/noripple_check.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct NoRippleCheck<'a> {
    /// A unique identifier for the account, most commonly the
    /// account's address.
    pub account: &'a str,
    /// Whether the address refers to a gateway or user.
    /// Recommendations depend on the role of the account.
    /// Issuers must have Default Ripple enabled and must disable
    /// No Ripple on all trust lines. Users should have Default Ripple
    /// disabled, and should enable No Ripple on all trust lines.
    pub role: NoRippleCheckRole,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// A 20-byte hex string for the ledger version to use.
    pub ledger_hash: Option<&'a str>,
    /// The ledger index of the ledger to use, or a shortcut string
    /// to choose a ledger automatically.
    pub ledger_index: Option<&'a str>,
    /// If true, include an array of suggested transactions, as JSON
    /// objects, that you can sign and submit to fix the problems.
    /// Defaults to false.
    pub transactions: Option<bool>,
    /// The maximum number of trust line problems to include in the
    /// results. Defaults to 300.
    pub limit: Option<u16>,
    /// The request method.
    #[serde(default = "RequestMethod::no_ripple_check")]
    pub command: RequestMethod,
}

impl Default for NoRippleCheck<'static> {
    fn default() -> Self {
        NoRippleCheck {
            account: "",
            role: Default::default(),
            id: None,
            ledger_hash: None,
            ledger_index: None,
            transactions: None,
            limit: None,
            command: RequestMethod::NoRippleCheck,
        }
    }
}

impl Model for NoRippleCheck<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `NoRippleCheck` to json.")
    }
}

/// WebSocket API only! The path_find method searches for
/// a path along which a transaction can possibly be made,
/// and periodically sends updates when the path changes
/// over time. For a simpler version that is supported by
/// JSON-RPC, see the ripple_path_find method. For payments
/// occurring strictly in XRP, it is not necessary to find
/// a path, because XRP can be sent directly to any account.
///
/// Although the rippled server tries to find the cheapest
/// path or combination of paths for making a payment, it is
/// not guaranteed that the paths returned by this method
/// are, in fact, the best paths. Due to server load,
/// pathfinding may not find the best results. Additionally,
/// you should be careful with the pathfinding results from
/// untrusted servers. A server could be modified to return
/// less-than-optimal paths to earn money for its operators.
/// If you do not have your own server that you can trust
/// with pathfinding, you should compare the results of
/// pathfinding from multiple servers run by different
/// parties, to minimize the risk of a single server
/// returning poor results. (Note: A server returning
/// less-than-optimal results is not necessarily proof of
/// malicious behavior; it could also be a symptom of heavy
/// server load.)
///
/// See Path Find:
/// `<https://xrpl.org/path_find.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct PathFind<'a> {
    /// Use "create" to send the create sub-command.
    pub subcommand: PathFindSubcommand,
    /// Unique address of the account to find a path
    /// from. (In other words, the account that would
    /// be sending a payment.)
    pub source_account: &'a str,
    /// Unique address of the account to find a path to.
    /// (In other words, the account that would receive a payment.)
    pub destination_account: &'a str,
    /// Currency Amount that the destination account would
    /// receive in a transaction. Special case: New in: rippled 0.30.0
    /// You can specify "-1" (for XRP) or provide -1 as the contents of
    /// the value field (for non-XRP currencies). This requests a path
    /// to deliver as much as possible, while spending no more than
    /// the amount specified in send_max (if provided).
    pub destination_amount: Currency,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// Currency Amount that would be spent in the transaction.
    /// Not compatible with source_currencies.
    pub send_max: Option<Currency>,
    /// Array of arrays of objects, representing payment paths to check.
    /// You can use this to keep updated on changes to particular paths
    /// you already know about, or to check the overall cost to make a
    /// payment along a certain path.
    pub paths: Option<Vec<Path<'a>>>,
    /// The request method.
    #[serde(default = "RequestMethod::path_find")]
    pub command: RequestMethod,
}

impl Default for PathFind<'static> {
    fn default() -> Self {
        PathFind {
            subcommand: Default::default(),
            source_account: "",
            destination_account: "",
            destination_amount: Currency::Xrp {
                value: None,
                currency: Default::default(),
            },
            id: None,
            send_max: None,
            paths: None,
            command: RequestMethod::PathFind,
        }
    }
}

impl Model for PathFind<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `PathFind` to json.")
    }
}

/// The ripple_path_find method is a simplified version of
/// the path_find method that provides a single response with
/// a payment path you can use right away. It is available in
/// both the WebSocket and JSON-RPC APIs. However, the
/// results tend to become outdated as time passes. Instead of
/// making multiple calls to stay updated, you should instead
/// use the path_find method to subscribe to continued updates
/// where possible.
///
/// Although the rippled server tries to find the cheapest path
/// or combination of paths for making a payment, it is not
/// guaranteed that the paths returned by this method are, in
/// fact, the best paths.
///
/// See Ripple Path Find:
/// `<https://xrpl.org/ripple_path_find.html#ripple_path_find>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RipplePathFind<'a> {
    /// Unique address of the account that would send funds
    /// in a transaction.
    pub source_account: &'a str,
    /// Unique address of the account that would receive funds
    /// in a transaction.
    pub destination_account: &'a str,
    /// Currency Amount that the destination account would
    /// receive in a transaction. Special case: New in: rippled 0.30.0
    /// You can specify "-1" (for XRP) or provide -1 as the contents
    /// of the value field (for non-XRP currencies). This requests a
    /// path to deliver as much as possible, while spending no more
    /// than the amount specified in send_max (if provided).
    pub destination_amount: Currency,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// A 20-byte hex string for the ledger version to use.
    pub ledger_hash: Option<&'a str>,
    /// The ledger index of the ledger to use, or a shortcut
    /// string to choose a ledger automatically.
    pub ledger_index: Option<&'a str>,
    /// Currency Amount that would be spent in the transaction.
    /// Cannot be used with source_currencies.
    pub send_max: Option<Currency>,
    /// Array of currencies that the source account might want
    /// to spend. Each entry in the array should be a JSON object
    /// with a mandatory currency field and optional issuer field,
    /// like how currency amounts are specified. Cannot contain
    /// more than 18 source currencies. By default, uses all source
    /// currencies available up to a maximum of 88 different
    /// currency/issuer pairs.
    pub source_currencies: Option<Vec<Currency>>,
    /// The request method.
    #[serde(default = "RequestMethod::ripple_path_find")]
    pub command: RequestMethod,
}

impl Default for RipplePathFind<'static> {
    fn default() -> Self {
        RipplePathFind {
            source_account: "",
            destination_account: "",
            destination_amount: Currency::Xrp {
                value: None,
                currency: Default::default(),
            },
            id: None,
            ledger_hash: None,
            ledger_index: None,
            send_max: None,
            source_currencies: None,
            command: RequestMethod::RipplePathFind,
        }
    }
}

impl Model for RipplePathFind<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `RipplePathFind` to json.")
    }
}

/// The ping command returns an acknowledgement, so that
/// clients can test the connection status and latency.
///
/// See Ping:
/// `<https://xrpl.org/ping.html#ping>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Ping<'a> {
    /// The unique request id.
    pub id: Option<&'a str>,
    /// The request method.
    #[serde(default = "RequestMethod::ping")]
    pub command: RequestMethod,
}

impl Default for Ping<'static> {
    fn default() -> Self {
        Ping {
            id: None,
            command: RequestMethod::Ping,
        }
    }
}

impl Model for Ping<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `Ping` to json.")
    }
}

/// The random command provides a random number to be used
/// as a source of entropy for random number generation
/// by clients.
///
/// See Random:
/// `<https://xrpl.org/random.html#random>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Random<'a> {
    /// The unique request id.
    pub id: Option<&'a str>,
    /// The request method.
    #[serde(default = "RequestMethod::random")]
    pub command: RequestMethod,
}

impl Default for Random<'static> {
    fn default() -> Self {
        Random {
            id: None,
            command: RequestMethod::Random,
        }
    }
}

impl Model for Random<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `Random` to json.")
    }
}

/// The server_info command asks the server for a
/// human-readable version of various information about the
/// rippled server being queried.
///
/// See Server Info:
/// `<https://xrpl.org/server_info.html#server_info>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerInfo<'a> {
    /// The unique request id.
    pub id: Option<&'a str>,
    /// The request info.
    #[serde(default = "RequestMethod::server_info")]
    pub command: RequestMethod,
}

impl Default for ServerInfo<'static> {
    fn default() -> Self {
        ServerInfo {
            id: None,
            command: RequestMethod::ServerInfo,
        }
    }
}

impl Model for ServerInfo<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `ServerInfo` to json.")
    }
}

/// The server_state command asks the server for various
/// machine-readable information about the rippled server's
/// current state. The response is almost the same as the
/// server_info method, but uses units that are easier to
/// process instead of easier to read. (For example, XRP
/// values are given in integer drops instead of scientific
/// notation or decimal values, and time is given in
/// milliseconds instead of seconds.)
///
/// See Server State:
/// `<https://xrpl.org/server_state.html#server_state>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerState<'a> {
    /// The unique request id.
    pub id: Option<&'a str>,
    /// The request method.
    #[serde(default = "RequestMethod::server_state")]
    pub command: RequestMethod,
}

impl Default for ServerState<'static> {
    fn default() -> Self {
        ServerState {
            id: None,
            command: RequestMethod::ServerState,
        }
    }
}

impl Model for ServerState<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `ServerState` to json.")
    }
}

/// The submit method applies a transaction and sends it to the
/// network to be confirmed and included in future ledgers.
///
/// This command has two modes:
/// * Submit-only mode takes a signed, serialized transaction as
///   a binary blob, and submits it to the network as-is. Since
///   signed transaction objects are immutable, no part of the
///   transaction can be modified or automatically filled in
///   after submission.
/// * Sign-and-submit mode takes a JSON-formatted Transaction
///   object, completes and signs the transaction in the same
///   manner as the sign method, and then submits the signed
///   transaction. We recommend only using this mode for
///   testing and development.
///
/// To send a transaction as robustly as possible, you should
/// construct and sign it in advance, persist it somewhere that
/// you can access even after a power outage, then submit it as a
/// tx_blob. After submission, monitor the network with the tx
/// method command to see if the transaction was successfully
/// applied; if a restart or other problem occurs, you can safely
/// re-submit the tx_blob transaction: it won't be applied twice
/// since it has the same sequence number as the old transaction.
///
/// See Submit:
/// `<https://xrpl.org/submit.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SignAndSubmit<'a> {
    // TODO
    // #[serde(rename(serialize = "tx_json", deserialize = "transaction"))]
    // transaction,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// Secret key of the account supplying the transaction, used to sign it.
    /// Do not send your secret to untrusted servers or through unsecured
    /// network connections. Cannot be used with key_type, seed, seed_hex,
    /// or passphrase.
    pub secret: Option<&'a str>,
    /// Secret key of the account supplying the transaction, used to sign it.
    /// Must be in the XRP Ledger's base58 format. If provided, you must also
    /// specify the key_type. Cannot be used with secret, seed_hex, or passphrase.
    pub seed: Option<&'a str>,
    /// Secret key of the account supplying the transaction, used to sign it.
    /// Must be in hexadecimal format. If provided, you must also specify the
    /// key_type. Cannot be used with secret, seed, or passphrase.
    pub seed_hex: Option<&'a str>,
    /// Secret key of the account supplying the transaction, used to sign it,
    /// as a string passphrase. If provided, you must also specify the key_type.
    /// Cannot be used with secret, seed, or seed_hex.
    pub passphrase: Option<&'a str>,
    /// Type of cryptographic key provided in this request. Valid types are
    /// secp256k1 or ed25519. Defaults to secp256k1. Cannot be used with secret.
    /// Caution: Ed25519 support is experimental.
    pub key_type: Option<CryptoAlgorithm>,
    /// If true, when constructing the transaction, do not try to automatically
    /// fill in or validate values.
    #[serde(default = "default_false")]
    pub offline: Option<bool>,
    /// If this field is provided, the server auto-fills the Paths field of a
    /// Payment transaction before signing. You must omit this field if the
    /// transaction is a direct XRP payment or if it is not a Payment-type
    /// transaction. Caution: The server looks for the presence or absence of
    /// this field, not its value. This behavior may change.
    pub build_path: Option<bool>,
    /// Sign-and-submit fails with the error rpcHIGH_FEE if the auto-filled
    /// Fee value would be greater than the reference
    /// transaction cost × fee_mult_max ÷ fee_div_max.
    /// This field has no effect if you explicitly specify the Fee field of
    /// the transaction. The default is 10.
    pub fee_mult_max: Option<u32>,
    /// Sign-and-submit fails with the error rpcHIGH_FEE if the auto-filled
    /// Fee value would be greater than the reference
    /// transaction cost × fee_mult_max ÷ fee_div_max.
    /// This field has no effect if you explicitly specify the Fee field of
    /// the transaction. The default is 1.
    pub fee_div_max: Option<u32>,
    /// If true, and the transaction fails locally, do not retry
    /// or relay the transaction to other servers
    #[serde(default = "default_false")]
    pub fail_hard: Option<bool>,
    /// The request method.
    #[serde(default = "RequestMethod::submit")]
    pub command: RequestMethod,
}

impl Default for SignAndSubmit<'static> {
    fn default() -> Self {
        SignAndSubmit {
            id: None,
            secret: None,
            seed: None,
            seed_hex: None,
            passphrase: None,
            key_type: None,
            offline: None,
            build_path: None,
            fee_mult_max: None,
            fee_div_max: None,
            fail_hard: None,
            command: RequestMethod::Submit,
        }
    }
}

impl Model for SignAndSubmit<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `SignAndSubmit` to json.")
    }

    fn get_errors(&self) -> Result<(), exceptions::XRPLModelException> {
        match self._get_field_error() {
            Err(error) => Err(XRPLModelException::XRPLRequestError(
                XRPLRequestException::SignAndSubmitError(error),
            )),
            Ok(_no_error) => match self._get_key_type_error() {
                Err(error) => Err(XRPLModelException::XRPLRequestError(
                    XRPLRequestException::SignAndSubmitError(error),
                )),
                Ok(_no_error) => Ok(()),
            },
        }
    }
}

impl SignAndSubmitError for SignAndSubmit<'static> {
    fn _get_field_error(&self) -> Result<(), SignAndSubmitException> {
        let mut signing_methods = Vec::new();
        for method in [self.secret, self.seed, self.seed_hex, self.passphrase] {
            if method.is_some() {
                signing_methods.push(method)
            }
        }
        match signing_methods.len() != 1 {
            true => Err(SignAndSubmitException::InvalidMustSetExactlyOneOf {
                fields: "`secret`, `seed`, `seed_hex`, `passphrase`".to_string(),
            }),
            false => Ok(()),
        }
    }

    fn _get_key_type_error(&self) -> Result<(), SignAndSubmitException> {
        match self.secret.is_some() && self.key_type.is_some() {
            true => Err(SignAndSubmitException::InvalidMustOmitKeyTypeIfSecretProvided),
            false => Ok(()),
        }
    }
}

/// The submit method applies a transaction and sends it to
/// the network to be confirmed and included in future ledgers.
///
/// This command has two modes:
/// * Submit-only mode takes a signed, serialized transaction
///   as a binary blob, and submits it to the network as-is.
///   Since signed transaction objects are immutable, no part
///   of the transaction can be modified or automatically
///   filled in after submission.
/// * Sign-and-submit mode takes a JSON-formatted Transaction
///   object, completes and signs the transaction in the same
///   manner as the sign method, and then submits the signed
///   transaction. We recommend only using this mode for
///   testing and development.
///
/// To send a transaction as robustly as possible, you should
/// construct and sign it in advance, persist it somewhere that
/// you can access even after a power outage, then submit it as
/// a tx_blob. After submission, monitor the network with the
/// tx method command to see if the transaction was successfully
/// applied; if a restart or other problem occurs, you can
/// safely re-submit the tx_blob transaction: it won't be
/// applied twice since it has the same sequence number as the
/// old transaction.
///
/// See Submit:
/// `<https://xrpl.org/submit.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitOnly<'a> {
    /// Hex representation of the signed transaction to submit.
    /// This can also be a multi-signed transaction.
    pub tx_blob: &'a str,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// If true, and the transaction fails locally, do not retry
    /// or relay the transaction to other servers
    #[serde(default = "default_false")]
    pub fail_hard: Option<bool>,
    /// The request method.
    #[serde(default = "RequestMethod::submit")]
    pub command: RequestMethod,
}

impl Default for SubmitOnly<'static> {
    fn default() -> Self {
        SubmitOnly {
            tx_blob: "",
            id: None,
            fail_hard: None,
            command: RequestMethod::Submit,
        }
    }
}

impl Model for SubmitOnly<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `SubmitOnly` to json.")
    }
}

/// The sign_for command provides one signature for a multi-signed
/// transaction. By default, this method is admin-only. It can be
/// used as a public method if the server has enabled public
/// signing. This command requires the MultiSign amendment to be
/// enabled.
///
/// See Sign For:
/// `<https://xrpl.org/sign_for.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SignFor<'a> {
    // TODO
    // #[serde(rename(serialize = "tx_json", deserialize = "transaction"))]
    // transaction,
    /// The address which is providing the signature.
    pub account: &'a str,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// Secret key of the account supplying the transaction, used
    /// to sign it. Do not send your secret to untrusted servers
    /// or through unsecured network connections. Cannot be used
    /// with key_type, seed, seed_hex, or passphrase.
    pub secret: Option<&'a str>,
    /// Secret key of the account supplying the transaction, used
    /// to sign it. Must be in the XRP Ledger's base58 format. If
    /// provided, you must also specify the key_type. Cannot be
    /// used with secret, seed_hex, or passphrase.
    pub seed: Option<&'a str>,
    /// Secret key of the account supplying the transaction, used
    /// to sign it. Must be in hexadecimal format. If provided,
    /// you must also specify the key_type. Cannot be used with
    /// secret, seed, or passphrase.
    pub seed_hex: Option<&'a str>,
    /// Secret key of the account supplying the transaction, used
    /// to sign it, as a string passphrase. If provided, you must
    /// also specify the key_type. Cannot be used with secret,
    /// seed, or seed_hex.
    pub passphrase: Option<&'a str>,
    /// Type of cryptographic key provided in this request. Valid
    /// types are secp256k1 or ed25519. Defaults to secp256k1.
    /// Cannot be used with secret. Caution: Ed25519 support is
    /// experimental.
    pub key_type: Option<CryptoAlgorithm>,
    /// The request method.
    #[serde(default = "RequestMethod::sign_for")]
    pub command: RequestMethod,
}

impl Default for SignFor<'static> {
    fn default() -> Self {
        SignFor {
            account: "",
            id: None,
            secret: None,
            seed: None,
            seed_hex: None,
            passphrase: None,
            key_type: None,
            command: RequestMethod::SignFor,
        }
    }
}

impl Model for SignFor<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `SignFor` to json.")
    }

    fn get_errors(&self) -> Result<(), exceptions::XRPLModelException> {
        match self._get_field_error() {
            Err(error) => Err(XRPLModelException::XRPLRequestError(
                XRPLRequestException::SignForError(error),
            )),
            Ok(_no_error) => match self._get_key_type_error() {
                Err(error) => Err(XRPLModelException::XRPLRequestError(
                    XRPLRequestException::SignForError(error),
                )),
                Ok(_no_error) => Ok(()),
            },
        }
    }
}

impl SignForError for SignFor<'static> {
    fn _get_field_error(&self) -> Result<(), SignForException> {
        let mut signing_methods = Vec::new();
        for method in [self.secret, self.seed, self.seed_hex, self.passphrase] {
            if method.is_some() {
                signing_methods.push(method)
            }
        }
        match signing_methods.len() != 1 {
            true => Err(SignForException::InvalidMustSetExactlyOneOf {
                fields: "`secret`, `seed`, `seed_hex`, `passphrase`".to_string(),
            }),
            false => Ok(()),
        }
    }

    fn _get_key_type_error(&self) -> Result<(), SignForException> {
        match self.secret.is_some() && self.key_type.is_some() {
            true => Err(SignForException::InvalidMustOmitKeyTypeIfSecretProvided),
            false => Ok(()),
        }
    }
}

/// The sign method takes a transaction in JSON format and a seed
/// value, and returns a signed binary representation of the
/// transaction. To contribute one signature to a multi-signed
/// transaction, use the sign_for method instead. By default, this
/// method is admin-only. It can be used as a public method if the
/// server has enabled public signing.
///
/// Caution:
/// Unless you run the rippled server yourself, you should do local
/// signing with RippleAPI instead of using this command. An
/// untrustworthy server could change the transaction before signing
/// it, or use your secret key to sign additional arbitrary
/// transactions as if they came from you.
///
/// See Sign:
/// `<https://xrpl.org/sign.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Sign<'a> {
    // TODO
    // #[serde(rename(serialize = "tx_json", deserialize = "transaction"))]
    // transaction,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// The secret seed of the account supplying the transaction,
    /// used to sign it. Do not send your secret to untrusted
    /// servers or through unsecured network connections. Cannot
    /// be used with key_type, seed, seed_hex, or passphrase.
    pub secret: Option<&'a str>,
    /// The secret seed of the account supplying the transaction,
    /// used to sign it. Must be in the XRP Ledger's base58
    /// format. If provided, you must also specify the key_type.
    /// Cannot be used with secret, seed_hex, or passphrase.
    pub seed: Option<&'a str>,
    /// The secret seed of the account supplying the transaction,
    /// used to sign it. Must be in hexadecimal format. If
    /// provided, you must also specify the key_type. Cannot be
    /// used with secret, seed, or passphrase.
    pub seed_hex: Option<&'a str>,
    /// The secret seed of the account supplying the transaction,
    /// used to sign it, as a string passphrase. If provided,
    /// you must also specify the key_type. Cannot be used with
    /// secret, seed, or seed_hex.
    pub passphrase: Option<&'a str>,
    /// The signing algorithm of the cryptographic key pair provided.
    /// Valid types are secp256k1 or ed25519. Defaults to secp256k1.
    /// Cannot be used with secret.
    pub key_type: Option<CryptoAlgorithm>,
    /// If true, when constructing the transaction, do not try to
    /// automatically fill any transaction details. The default
    /// is false.
    #[serde(default = "default_false")]
    pub offline: Option<bool>,
    /// If this field is provided, the server auto-fills the Paths
    /// field of a Payment transaction before signing. You must omit
    /// this field if the transaction is a direct XRP payment or if
    /// it is not a Payment-type transaction. Caution: The server
    /// looks for the presence or absence of this field, not its value.
    /// This behavior may change.
    pub build_path: Option<bool>,
    /// Signing fails with the error rpcHIGH_FEE if the auto-filled
    /// Fee value would be greater than the reference
    /// transaction cost × fee_mult_max ÷ fee_div_max. This field has
    /// no effect if you explicitly specify the Fee field of the
    /// transaction. The default is 10.
    pub fee_mult_max: Option<u32>,
    /// Signing fails with the error rpcHIGH_FEE if the auto-filled
    /// Fee value would be greater than the reference
    /// transaction cost × fee_mult_max ÷ fee_div_max. This field has
    /// no effect if you explicitly specify the Fee field of the
    /// transaction. The default is 1.
    pub fee_div_max: Option<u32>,
    /// The request method.
    #[serde(default = "RequestMethod::sign")]
    pub command: RequestMethod,
}

impl Default for Sign<'static> {
    fn default() -> Self {
        Sign {
            id: None,
            secret: None,
            seed: None,
            seed_hex: None,
            passphrase: None,
            key_type: None,
            offline: None,
            build_path: None,
            fee_mult_max: None,
            fee_div_max: None,
            command: RequestMethod::Sign,
        }
    }
}

impl Model for Sign<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `Sign` to json.")
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self._get_field_error() {
            Err(error) => Err(XRPLModelException::XRPLRequestError(
                XRPLRequestException::SignError(error),
            )),
            Ok(_no_error) => match self._get_key_type_error() {
                Err(error) => Err(XRPLModelException::XRPLRequestError(
                    XRPLRequestException::SignError(error),
                )),
                Ok(_no_error) => Ok(()),
            },
        }
    }
}

impl SignError for Sign<'static> {
    fn _get_field_error(&self) -> Result<(), SignException> {
        let mut signing_methods = Vec::new();
        for method in [self.secret, self.seed, self.seed_hex, self.passphrase] {
            if method.is_some() {
                signing_methods.push(method)
            }
        }
        match signing_methods.len() != 1 {
            true => Err(SignException::InvalidMustSetExactlyOneOf {
                fields: "`secret`, `seed`, `seed_hex`, `passphrase`".to_string(),
            }),
            false => Ok(()),
        }
    }

    fn _get_key_type_error(&self) -> Result<(), SignException> {
        match self.secret.is_some() && self.key_type.is_some() {
            true => Err(SignException::InvalidMustOmitKeyTypeIfSecretProvided),
            false => Ok(()),
        }
    }
}

/// The server_state command asks the server for various
/// machine-readable information about the rippled server's
/// current state. The response is almost the same as the
/// server_info method, but uses units that are easier to
/// process instead of easier to read. (For example, XRP
/// values are given in integer drops instead of scientific
/// notation or decimal values, and time is given in
/// milliseconds instead of seconds.)
///
/// See Submit Multisigned:
/// `<https://xrpl.org/submit_multisigned.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitMultisigned<'a> {
    // TODO
    // #[serde(rename(serialize = "tx_json", deserialize = "transaction"))]
    // transaction,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// If true, and the transaction fails locally, do not
    /// retry or relay the transaction to other servers.
    #[serde(default = "default_false")]
    pub fail_hard: Option<bool>,
    /// The request method.
    #[serde(default = "RequestMethod::submit_multisigned")]
    pub command: RequestMethod,
}

impl Default for SubmitMultisigned<'static> {
    fn default() -> Self {
        SubmitMultisigned {
            id: None,
            fail_hard: None,
            command: RequestMethod::SubmitMultisigned,
        }
    }
}

impl Model for SubmitMultisigned<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `SubmitMultisigned` to json.")
    }
}

/// The subscribe method requests periodic notifications
/// from the server when certain events happen.
///
/// Note: WebSocket API only.
///
/// See Subscribe:
/// `<https://xrpl.org/subscribe.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Subscribe<'a> {
    /// The unique request id.
    pub id: Option<&'a str>,
    /// Array of objects defining order books  to monitor for
    /// updates, as detailed below.
    pub books: Option<Vec<SubscribeBookFields<'a>>>,
    /// Array of string names of generic streams to subscribe to.
    pub streams: Option<Vec<StreamParameter>>,
    /// Array with the unique addresses of accounts to monitor
    /// for validated transactions. The addresses must be in the
    /// XRP Ledger's base58 format. The server sends a notification
    /// for any transaction that affects at least one of these accounts.
    pub accounts: Option<Vec<&'a str>>,
    /// Like accounts, but include transactions that are not
    /// yet finalized.
    pub accounts_proposed: Option<Vec<&'a str>>,
    /// (Optional for Websocket; Required otherwise) URL where the server
    /// sends a JSON-RPC callbacks for each event. Admin-only.
    pub url: Option<&'a str>,
    /// Username to provide for basic authentication at the callback URL.
    pub url_username: Option<&'a str>,
    /// Password to provide for basic authentication at the callback URL.
    pub url_password: Option<&'a str>,
    /// The request method.
    // #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::subscribe")]
    pub command: RequestMethod,
}

impl Default for Subscribe<'static> {
    fn default() -> Self {
        Subscribe {
            id: None,
            books: None,
            streams: None,
            accounts: None,
            accounts_proposed: None,
            url: None,
            url_username: None,
            url_password: None,
            command: RequestMethod::Subscribe,
        }
    }
}

impl Model for Subscribe<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `Subscribe` to json.")
    }
}

/// The unsubscribe command tells the server to stop
/// sending messages for a particular subscription or set
/// of subscriptions.
///
/// Note: WebSocket API only.
///
/// See Unsubscribe:
/// `<https://xrpl.org/unsubscribe.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Unsubscribe<'a> {
    /// The unique request id.
    pub id: Option<&'a str>,
    /// Array of objects defining order books to unsubscribe
    /// from, as explained below.
    pub books: Option<Vec<UnsubscribeBookFields>>,
    /// Array of string names of generic streams to unsubscribe
    /// from, including ledger, server, transactions,
    /// and transactions_proposed.
    pub streams: Option<Vec<StreamParameter>>,
    /// Array of unique account addresses to stop receiving updates
    /// for, in the XRP Ledger's base58 format. (This only stops
    /// those messages if you previously subscribed to those accounts
    /// specifically. You cannot use this to filter accounts out of
    /// the general transactions stream.)
    pub accounts: Option<Vec<&'a str>>,
    /// Like accounts, but for accounts_proposed subscriptions that
    /// included not-yet-validated transactions.
    pub accounts_proposed: Option<Vec<&'a str>>,
    // TODO Lifetime issue
    #[serde(skip_serializing)]
    pub broken: Option<&'a str>,
    /// The request method.
    #[serde(default = "RequestMethod::unsubscribe")]
    pub command: RequestMethod,
}

impl Default for Unsubscribe<'static> {
    fn default() -> Self {
        Unsubscribe {
            id: None,
            books: None,
            streams: None,
            accounts: None,
            accounts_proposed: None,
            broken: None,
            command: RequestMethod::Unsubscribe,
        }
    }
}

impl Model for Unsubscribe<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `Unsubscribe` to json.")
    }
}

/// The transaction_entry method retrieves information on a
/// single transaction from a specific ledger version.
/// (The tx method, by contrast, searches all ledgers for
/// the specified transaction. We recommend using that
/// method instead.)
///
/// See Transaction Entry:
/// `<https://xrpl.org/transaction_entry.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionEntry<'a> {
    /// Unique hash of the transaction you are looking up.
    pub tx_hash: &'a str,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// A 20-byte hex string for the ledger version to use.
    pub ledger_hash: Option<&'a str>,
    /// The ledger index of the ledger to use, or a shortcut
    /// string to choose a ledger automatically.
    pub ledger_index: Option<&'a str>,
    /// The request method.
    #[serde(default = "RequestMethod::transaction_entry")]
    pub command: RequestMethod,
}

impl Default for TransactionEntry<'static> {
    fn default() -> Self {
        TransactionEntry {
            tx_hash: "",
            id: None,
            ledger_hash: None,
            ledger_index: None,
            command: RequestMethod::TransactionEntry,
        }
    }
}

impl Model for TransactionEntry<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `TransactionEntry` to json.")
    }
}

/// The tx method retrieves information on a single transaction.
///
/// See Tx:
/// `<https://xrpl.org/tx.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Tx<'a> {
    // TODO
    // #[serde(rename(serialize = "tx_json", deserialize = "transaction"))]
    // transaction,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// If true, return transaction data and metadata as binary
    /// serialized to hexadecimal strings. If false, return
    /// transaction data and metadata as JSON. The default is false.
    #[serde(default = "default_false")]
    pub binary: Option<bool>,
    /// Use this with max_ledger to specify a range of up to 1000
    /// ledger indexes, starting with this ledger (inclusive). If
    /// the server cannot find the transaction, it confirms whether
    /// it was able to search all the ledgers in this range.
    pub min_ledger: Option<u32>,
    /// Use this with min_ledger to specify a range of up to 1000
    /// ledger indexes, ending with this ledger (inclusive). If the
    /// server cannot find the transaction, it confirms whether it
    /// was able to search all the ledgers in the requested range.
    pub max_ledger: Option<u32>,
    /// The request method.
    #[serde(default = "RequestMethod::tx")]
    pub command: RequestMethod,
}

impl Default for Tx<'static> {
    fn default() -> Self {
        Tx {
            id: None,
            binary: None,
            min_ledger: None,
            max_ledger: None,
            command: RequestMethod::Tx,
        }
    }
}

impl Model for Tx<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `Tx` to json.")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_deserialize_account_channels() {
        let json = r#"{"account":"rH6ZiHU1PGamME2LvVTxrgvfjQpppWKGmr","command":"account_channels","marker":12345678}"#;
        let test: AccountChannels = serde_json::from_str(json).expect("");
        let expect: AccountChannels = AccountChannels {
            command: RequestMethod::AccountChannels,
            account: "rH6ZiHU1PGamME2LvVTxrgvfjQpppWKGmr",
            marker: Some(12345678),
            id: None,
            ledger_hash: None,
            ledger_index: None,
            limit: None,
            destination_account: None,
        };
        let revert = expect.to_json();

        assert_eq!(test, expect);
        assert_eq!(revert, json);
    }
}

#[cfg(test)]
mod test_channel_authorize_errors {
    use alloc::string::ToString;

    use crate::{
        constants::CryptoAlgorithm,
        models::{
            exceptions::{ChannelAuthorizeException, XRPLModelException, XRPLRequestException},
            Model, RequestMethod,
        },
    };

    use super::ChannelAuthorize;

    #[test]
    fn test_fields_error() {
        let channel_authorize = ChannelAuthorize {
            command: RequestMethod::ChannelAuthorize,
            channel_id: "5DB01B7FFED6B67E6B0414DED11E051D2EE2B7619CE0EAA6286D67A3A4D5BDB3",
            amount: "1000000",
            id: None,
            secret: None,
            seed: Some(""),
            seed_hex: Some(""),
            passphrase: None,
            key_type: Some(CryptoAlgorithm::SECP256K1),
        };
        let expected_error =
            XRPLModelException::XRPLRequestError(XRPLRequestException::ChannelAuthorizeError(
                ChannelAuthorizeException::InvalidMustSetExactlyOneOf {
                    fields: "`secret`, `seed`, `seed_hex`, `passphrase`".to_string(),
                },
            ));
        assert_eq!(channel_authorize.validate(), Err(expected_error))
    }
}

#[cfg(test)]
mod test_ledger_entry_errors {
    use alloc::string::ToString;

    use crate::models::{
        exceptions::{LedgerEntryException, XRPLModelException, XRPLRequestException},
        request_fields::OfferFields,
        Model, RequestMethod,
    };

    use super::LedgerEntry;

    #[test]
    fn test_fields_error() {
        let ledger_entry = LedgerEntry {
            command: RequestMethod::LedgerEntry,
            id: None,
            index: None,
            account_root: Some("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn"),
            check: None,
            payment_channel: None,
            deposit_preauth: None,
            directory: None,
            escrow: None,
            offer: Some(OfferFields {
                account: "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
                seq: 359,
            }),
            ripple_state: None,
            ticket: None,
            binary: None,
            ledger_hash: None,
            ledger_index: None,
        };
        let expected_error = XRPLModelException::XRPLRequestError(XRPLRequestException::LedgerEntryError(LedgerEntryException::InvalidMustSetExactlyOneOf { fields: "`index`, `account_root`, `check`, `directory`, `offer`, `ripple_state`, `escrow`, `payment_channel`, `deposit_preauth`, `ticket`".to_string() }));
        assert_eq!(ledger_entry.validate(), Err(expected_error))
    }
}

#[cfg(test)]
mod test_sign_and_submit_errors {
    use alloc::string::ToString;

    use crate::{
        constants::CryptoAlgorithm,
        models::{
            exceptions::{SignAndSubmitException, XRPLModelException, XRPLRequestException},
            Model, RequestMethod,
        },
    };

    use super::SignAndSubmit;

    #[test]
    fn test_fields_error() {
        let mut sign_and_submit = SignAndSubmit {
            command: RequestMethod::Submit,
            id: None,
            secret: Some(""),
            seed: Some(""),
            seed_hex: None,
            passphrase: None,
            key_type: None,
            offline: None,
            build_path: None,
            fee_mult_max: None,
            fee_div_max: None,
            fail_hard: None,
        };
        let expected_error =
            XRPLModelException::XRPLRequestError(XRPLRequestException::SignAndSubmitError(
                SignAndSubmitException::InvalidMustSetExactlyOneOf {
                    fields: "`secret`, `seed`, `seed_hex`, `passphrase`".to_string(),
                },
            ));
        assert_eq!(sign_and_submit.validate(), Err(expected_error));

        sign_and_submit.seed = None;
        sign_and_submit.key_type = Some(CryptoAlgorithm::SECP256K1);
        let expected_error =
            XRPLModelException::XRPLRequestError(XRPLRequestException::SignAndSubmitError(
                SignAndSubmitException::InvalidMustOmitKeyTypeIfSecretProvided,
            ));
        assert_eq!(sign_and_submit.validate(), Err(expected_error));
    }
}

#[cfg(test)]
mod test_sign_for_errors {
    use alloc::string::ToString;

    use crate::{
        constants::CryptoAlgorithm,
        models::{
            exceptions::{SignForException, XRPLModelException, XRPLRequestException},
            Model, RequestMethod,
        },
    };

    use super::SignFor;

    #[test]
    fn test_fields_error() {
        let mut sign_for = SignFor {
            command: RequestMethod::SignFor,
            account: "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
            id: None,
            secret: Some(""),
            seed: Some(""),
            seed_hex: None,
            passphrase: None,
            key_type: None,
        };
        let expected_error = XRPLModelException::XRPLRequestError(
            XRPLRequestException::SignForError(SignForException::InvalidMustSetExactlyOneOf {
                fields: "`secret`, `seed`, `seed_hex`, `passphrase`".to_string(),
            }),
        );
        assert_eq!(sign_for.validate(), Err(expected_error));

        sign_for.seed = None;
        sign_for.key_type = Some(CryptoAlgorithm::SECP256K1);
        let expected_error =
            XRPLModelException::XRPLRequestError(XRPLRequestException::SignForError(
                SignForException::InvalidMustOmitKeyTypeIfSecretProvided,
            ));
        assert_eq!(sign_for.validate(), Err(expected_error));
    }
}

#[cfg(test)]
mod test_sign_errors {
    use alloc::string::ToString;

    use crate::{
        constants::CryptoAlgorithm,
        models::{
            exceptions::{SignException, XRPLModelException, XRPLRequestException},
            Model, RequestMethod,
        },
    };

    use super::Sign;

    #[test]
    fn test_fields_error() {
        let mut sign = Sign {
            command: RequestMethod::Sign,
            id: None,
            secret: Some(""),
            seed: Some(""),
            seed_hex: None,
            passphrase: None,
            key_type: None,
            offline: None,
            build_path: None,
            fee_mult_max: None,
            fee_div_max: None,
        };
        let expected_error = XRPLModelException::XRPLRequestError(XRPLRequestException::SignError(
            SignException::InvalidMustSetExactlyOneOf {
                fields: "`secret`, `seed`, `seed_hex`, `passphrase`".to_string(),
            },
        ));
        assert_eq!(sign.validate(), Err(expected_error));

        sign.seed = None;
        sign.key_type = Some(CryptoAlgorithm::SECP256K1);
        let expected_error = XRPLModelException::XRPLRequestError(XRPLRequestException::SignError(
            SignException::InvalidMustOmitKeyTypeIfSecretProvided,
        ));
        assert_eq!(sign.validate(), Err(expected_error));
    }
}
