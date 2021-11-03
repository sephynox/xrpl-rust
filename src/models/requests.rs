//! Request models.

use crate::constants::CryptoAlgorithm;
use crate::core::types::Amount;
use crate::core::types::PathSet;
use crate::models::*;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

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
    directory: Option<Directory<'a>>,
    escrow: Option<Escrow<'a>>,
    offer: Option<Offer<'a>>,
    ripple_state: Option<RippleState<'a>>,
    ticket: Option<Ticket<'a>>,
    #[serde(default = "default_false")]
    binary: Option<bool>,
    ledger_hash: Option<&'a str>,
    ledger_index: Option<&'a str>,
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
    destination_amount: Amount,
    id: Option<u32>,
    send_max: Option<Amount>,
    paths: Option<PathSet>,
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
    destination_amount: Amount,
    id: Option<u32>,
    ledger_hash: Option<&'a str>,
    ledger_index: Option<&'a str>,
    send_max: Option<Amount>,
    source_currencies: Option<Vec<Currency>>,
    paths: Option<PathSet>,
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
    books: Option<Vec<SubscribeBook<'a>>>,
    streams: Option<Vec<StreamParameter>>,
    accounts: Option<Vec<&'a str>>,
    accounts_proposed: Option<Vec<&'a str>>,
    url: Option<&'a str>,
    url_username: Option<&'a str>,
    url_password: Option<&'a str>,
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
    books: Option<Vec<UnsubscribeBook>>,
    streams: Option<Vec<StreamParameter>>,
    accounts: Option<Vec<&'a str>>,
    accounts_proposed: Option<Vec<&'a str>>,
    // TODO Lifetime issue
    #[serde(skip_serializing)]
    broken: Option<&'a str>,
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
