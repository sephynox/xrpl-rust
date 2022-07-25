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
/// account is the channel's source, not thedestination.
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
/// let json = r#"{"account":"rH6ZiHU1PGamME2LvVTxrgvfjQpppWKGmr","marker":"something"}"#.to_string();
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::account_channels")]
    method: RequestMethod,
    account: &'a str,
    id: Option<u32>,
    ledger_hash: Option<&'a str>,
    ledger_index: Option<&'a str>,
    limit: Option<u16>,
    destination_account: Option<&'a str>,
    marker: Option<&'a str>,
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::account_currencies")]
    method: RequestMethod,
    account: &'a str,
    id: Option<u32>,
    ledger_hash: Option<&'a str>,
    ledger_index: Option<&'a str>,
    destination_account: Option<&'a str>,
    #[serde(default = "default_false")]
    strict: Option<bool>,
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::account_info")]
    method: RequestMethod,
    account: &'a str,
    id: Option<u32>,
    ledger_hash: Option<&'a str>,
    ledger_index: Option<&'a str>,
    #[serde(default = "default_false")]
    strict: Option<bool>,
    #[serde(default = "default_false")]
    queue: Option<bool>,
    #[serde(default = "default_false")]
    signer_lists: Option<bool>,
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::account_lines")]
    method: RequestMethod,
    account: &'a str,
    id: Option<u32>,
    ledger_hash: Option<&'a str>,
    ledger_index: Option<&'a str>,
    limit: Option<u16>,
    peer: Option<&'a str>,
    marker: Option<&'a str>,
}

impl Model for AccountLines<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `AccountLines` to json.")
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::account_objects")]
    method: RequestMethod,
    account: &'a str,
    id: Option<u32>,
    ledger_hash: Option<&'a str>,
    ledger_index: Option<&'a str>,
    r#type: Option<AccountObjectType>,
    #[serde(default = "default_false")]
    deletion_blockers_only: Option<bool>,
    limit: Option<u16>,
    marker: Option<&'a str>,
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::account_offers")]
    method: RequestMethod,
    account: &'a str,
    id: Option<u32>,
    ledger_hash: Option<&'a str>,
    ledger_index: Option<&'a str>,
    limit: Option<u16>,
    #[serde(default = "default_false")]
    strict: Option<bool>,
    marker: Option<&'a str>,
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::account_tx")]
    method: RequestMethod,
    account: &'a str,
    id: Option<u32>,
    ledger_hash: Option<&'a str>,
    ledger_index: Option<&'a str>,
    #[serde(default = "default_false")]
    binary: Option<bool>,
    #[serde(default = "default_false")]
    forward: Option<bool>,
    ledger_index_min: Option<u32>,
    ledger_index_max: Option<u32>,
    limit: Option<u16>,
    marker: Option<&'a str>,
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::book_offers")]
    method: RequestMethod,
    taker_gets: Currency,
    taker_pays: Currency,
    id: Option<u32>,
    ledger_hash: Option<&'a str>,
    ledger_index: Option<&'a str>,
    limit: Option<u16>,
    taker: Option<&'a str>,
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::channel_authorize")]
    method: RequestMethod,
    channel_id: &'a str,
    amount: &'a str,
    id: Option<u32>,
    secret: Option<&'a str>,
    seed: Option<&'a str>,
    seed_hex: Option<&'a str>,
    passphrase: Option<&'a str>,
    key_type: Option<CryptoAlgorithm>,
}

impl Model for ChannelAuthorize<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `ChannelAuthorize` to json.")
    }

    fn get_errors(&self) -> Result<(), exceptions::XRPLModelException> {
        match self.get_field_error() {
            Err(error) => Err(XRPLModelException::XRPLRequestError(
                XRPLRequestException::ChannelAuthorizeError(error),
            )),
            Ok(_no_error) => Ok(()),
        }
    }
}

impl ChannelAuthorizeError for ChannelAuthorize<'static> {
    fn get_field_error(&self) -> Result<(), ChannelAuthorizeException> {
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::channel_verify")]
    method: RequestMethod,
    channel_id: &'a str,
    amount: &'a str,
    public_key: &'a str,
    signature: &'a str,
    id: Option<u32>,
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::deposit_authorization")]
    method: RequestMethod,
    source_account: &'a str,
    destination_account: &'a str,
    id: Option<u32>,
    ledger_hash: Option<&'a str>,
    ledger_index: Option<&'a str>,
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
pub struct Fee {
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::fee")]
    method: RequestMethod,
    id: Option<u32>,
}

impl Model for Fee {
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::deposit_authorization")]
    method: RequestMethod,
    account: &'a str,
    id: Option<u32>,
    #[serde(default = "default_false")]
    strict: Option<bool>,
    ledger_hash: Option<&'a str>,
    ledger_index: Option<&'a str>,
    hotwallet: Option<Vec<&'a str>>,
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::ledger_closed")]
    method: RequestMethod,
    id: Option<u32>,
    ledger_hash: Option<&'a str>,
    ledger_index: Option<&'a str>,
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
pub struct LedgerCurrent {
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::ledger_current")]
    method: RequestMethod,
    id: Option<u32>,
}

impl Model for LedgerCurrent {
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::ledger_data")]
    method: RequestMethod,
    id: Option<u32>,
    ledger_hash: Option<&'a str>,
    ledger_index: Option<&'a str>,
    #[serde(default = "default_false")]
    binary: Option<bool>,
    limit: Option<u16>,
    marker: Option<&'a str>,
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::ledger_entry")]
    method: RequestMethod,
    id: Option<u32>,
    index: Option<&'a str>,
    account_root: Option<&'a str>,
    check: Option<&'a str>,
    payment_channel: Option<&'a str>,
    deposit_preauth: Option<DepositPreauth<'a>>,
    directory: Option<DirectoryFields<'a>>,
    escrow: Option<EscrowFields<'a>>,
    offer: Option<OfferFields<'a>>,
    ripple_state: Option<RippleStateFields<'a>>,
    ticket: Option<TicketFields<'a>>,
    #[serde(default = "default_false")]
    binary: Option<bool>,
    ledger_hash: Option<&'a str>,
    ledger_index: Option<&'a str>,
}

impl Model for LedgerEntry<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `LedgerEntry` to json.")
    }

    fn get_errors(&self) -> Result<(), exceptions::XRPLModelException> {
        match self.get_field_error() {
            Err(error) => Err(XRPLModelException::XRPLRequestError(
                XRPLRequestException::LedgerEntryError(error),
            )),
            Ok(_no_error) => Ok(()),
        }
    }
}

impl LedgerEntryError for LedgerEntry<'static> {
    fn get_field_error(&self) -> Result<(), LedgerEntryException> {
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::ledger")]
    method: RequestMethod,
    id: Option<u32>,
    ledger_hash: Option<&'a str>,
    ledger_index: Option<&'a str>,
    #[serde(default = "default_false")]
    full: Option<bool>,
    #[serde(default = "default_false")]
    accounts: Option<bool>,
    #[serde(default = "default_false")]
    transactions: Option<bool>,
    #[serde(default = "default_false")]
    expand: Option<bool>,
    #[serde(default = "default_false")]
    owner_funds: Option<bool>,
    #[serde(default = "default_false")]
    binary: Option<bool>,
    #[serde(default = "default_false")]
    queue: Option<bool>,
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::manifest")]
    method: RequestMethod,
    public_key: &'a str,
    id: Option<u32>,
}

impl Model for Manifest<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `Manifest` to json.")
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::no_ripple_check")]
    method: RequestMethod,
    account: &'a str,
    role: NoRippleCheckRole,
    id: Option<u32>,
    ledger_hash: Option<&'a str>,
    ledger_index: Option<&'a str>,
    transactions: Option<bool>,
    limit: Option<u16>,
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::path_find")]
    method: RequestMethod,
    subcommand: PathFindSubcommand,
    source_account: &'a str,
    destination_account: &'a str,
    destination_amount: Currency,
    id: Option<u32>,
    send_max: Option<Currency>,
    paths: Option<Vec<Path<'a>>>,
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::ripple_path_find")]
    method: RequestMethod,
    source_account: &'a str,
    destination_account: &'a str,
    destination_amount: Currency,
    id: Option<u32>,
    ledger_hash: Option<&'a str>,
    ledger_index: Option<&'a str>,
    send_max: Option<Currency>,
    source_currencies: Option<Vec<Currency>>,
    paths: Option<Vec<Vec<PathStep<'a>>>>,
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
pub struct Ping {
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::ping")]
    method: RequestMethod,
    id: Option<u32>,
}

impl Model for Ping {
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
pub struct Random {
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::random")]
    method: RequestMethod,
    id: Option<u32>,
}

impl Model for Random {
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
pub struct ServerInfo {
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::server_info")]
    method: RequestMethod,
    id: Option<u32>,
}

impl Model for ServerInfo {
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
pub struct ServerState {
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::server_state")]
    method: RequestMethod,
    id: Option<u32>,
}

impl Model for ServerState {
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::submit")]
    method: RequestMethod,
    // TODO
    // #[serde(rename(serialize = "tx_json", deserialize = "transaction"))]
    // transaction,
    id: Option<u32>,
    secret: Option<&'a str>,
    seed: Option<&'a str>,
    seed_hex: Option<&'a str>,
    passphrase: Option<&'a str>,
    key_type: Option<CryptoAlgorithm>,
    #[serde(default = "default_false")]
    offline: Option<bool>,
    build_path: Option<bool>,
    fee_mult_max: Option<u32>,
    fee_div_max: Option<u32>,
}

impl Model for SignAndSubmit<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `SignAndSubmit` to json.")
    }

    fn get_errors(&self) -> Result<(), exceptions::XRPLModelException> {
        match self.get_field_error() {
            Err(error) => Err(XRPLModelException::XRPLRequestError(
                XRPLRequestException::SignAndSubmitError(error),
            )),
            Ok(_no_error) => match self.get_key_type_error() {
                Err(error) => Err(XRPLModelException::XRPLRequestError(
                    XRPLRequestException::SignAndSubmitError(error),
                )),
                Ok(_no_error) => Ok(()),
            },
        }
    }
}

impl SignAndSubmitError for SignAndSubmit<'static> {
    fn get_field_error(&self) -> Result<(), SignAndSubmitException> {
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

    fn get_key_type_error(&self) -> Result<(), SignAndSubmitException> {
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::submit")]
    method: RequestMethod,
    tx_blob: &'a str,
    id: Option<u32>,
    #[serde(default = "default_false")]
    fail_hard: Option<bool>,
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::sign_for")]
    method: RequestMethod,
    // TODO
    // #[serde(rename(serialize = "tx_json", deserialize = "transaction"))]
    // transaction,
    account: &'a str,
    id: Option<u32>,
    secret: Option<&'a str>,
    seed: Option<&'a str>,
    seed_hex: Option<&'a str>,
    passphrase: Option<&'a str>,
    key_type: Option<CryptoAlgorithm>,
}

impl Model for SignFor<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `SignFor` to json.")
    }

    fn get_errors(&self) -> Result<(), exceptions::XRPLModelException> {
        match self.get_field_error() {
            Err(error) => Err(XRPLModelException::XRPLRequestError(
                XRPLRequestException::SignForError(error),
            )),
            Ok(_no_error) => match self.get_key_type_error() {
                Err(error) => Err(XRPLModelException::XRPLRequestError(
                    XRPLRequestException::SignForError(error),
                )),
                Ok(_no_error) => Ok(()),
            },
        }
    }
}

impl SignForError for SignFor<'static> {
    fn get_field_error(&self) -> Result<(), SignForException> {
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

    fn get_key_type_error(&self) -> Result<(), SignForException> {
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::sign")]
    method: RequestMethod,
    // TODO
    // #[serde(rename(serialize = "tx_json", deserialize = "transaction"))]
    // transaction,
    id: Option<u32>,
    secret: Option<&'a str>,
    seed: Option<&'a str>,
    seed_hex: Option<&'a str>,
    passphrase: Option<&'a str>,
    key_type: Option<CryptoAlgorithm>,
    #[serde(default = "default_false")]
    offline: Option<bool>,
    /// None does have meaning here
    build_path: Option<bool>,
    fee_mult_max: Option<u32>,
    fee_div_max: Option<u32>,
}

impl Model for Sign<'static> {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `Sign` to json.")
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_field_error() {
            Err(error) => Err(XRPLModelException::XRPLRequestError(
                XRPLRequestException::SignError(error),
            )),
            Ok(_no_error) => match self.get_key_type_error() {
                Err(error) => Err(XRPLModelException::XRPLRequestError(
                    XRPLRequestException::SignError(error),
                )),
                Ok(_no_error) => Ok(()),
            },
        }
    }
}

impl SignError for Sign<'static> {
    fn get_field_error(&self) -> Result<(), SignException> {
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

    fn get_key_type_error(&self) -> Result<(), SignException> {
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
pub struct SubmitMultisigned {
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::submit_multisigned")]
    method: RequestMethod,
    // TODO
    // #[serde(rename(serialize = "tx_json", deserialize = "transaction"))]
    // transaction,
    id: Option<u32>,
    #[serde(default = "default_false")]
    fail_hard: Option<bool>,
}

impl Model for SubmitMultisigned {
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::subscribe")]
    method: RequestMethod,
    id: Option<u32>,
    books: Option<Vec<SubscribeBookFields<'a>>>,
    streams: Option<Vec<StreamParameter>>,
    accounts: Option<Vec<&'a str>>,
    accounts_proposed: Option<Vec<&'a str>>,
    url: Option<&'a str>,
    url_username: Option<&'a str>,
    url_password: Option<&'a str>,
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::unsubscribe")]
    method: RequestMethod,
    id: Option<u32>,
    books: Option<Vec<UnsubscribeBookFields>>,
    streams: Option<Vec<StreamParameter>>,
    accounts: Option<Vec<&'a str>>,
    accounts_proposed: Option<Vec<&'a str>>,
    // TODO Lifetime issue
    #[serde(skip_serializing)]
    broken: Option<&'a str>,
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
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::transaction_entry")]
    method: RequestMethod,
    tx_hash: &'a str,
    id: Option<u32>,
    ledger_hash: Option<&'a str>,
    ledger_index: Option<&'a str>,
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
pub struct Tx {
    #[serde(skip_serializing)]
    #[serde(default = "RequestMethod::tx")]
    method: RequestMethod,
    // TODO
    // #[serde(rename(serialize = "tx_json", deserialize = "transaction"))]
    // transaction,
    id: Option<u32>,
    #[serde(default = "default_false")]
    binary: Option<bool>,
    min_ledger: Option<u32>,
    max_ledger: Option<u32>,
}

impl Model for Tx {
    fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("Unable to serialize `Tx` to json.")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_deserialize_account_channels() {
        let json = r#"{"account":"rH6ZiHU1PGamME2LvVTxrgvfjQpppWKGmr","marker":"something"}"#;
        let test: AccountChannels = serde_json::from_str(json).expect("");
        let expect: AccountChannels = AccountChannels {
            method: RequestMethod::AccountChannels,
            account: "rH6ZiHU1PGamME2LvVTxrgvfjQpppWKGmr",
            marker: Some("something"),
            id: None,
            ledger_hash: None,
            ledger_index: None,
            limit: None,
            destination_account: None,
        };
        let revert = serde_json::to_string(&expect);

        assert_eq!(test, expect);
        assert_eq!(revert.unwrap(), json);
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
            method: RequestMethod::ChannelAuthorize,
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
            method: RequestMethod::LedgerEntry,
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
            method: RequestMethod::Submit,
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
            method: RequestMethod::SignFor,
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
            method: RequestMethod::Sign,
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
