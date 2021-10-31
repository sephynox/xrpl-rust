//! Request models.

use crate::constants::CryptoAlgorithm;
use crate::models::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This request returns information about an account's
/// Payment Channels. This includes only channels where
/// the specified account is the channel's source, not
/// thedestination. (A channel's "source" and "owner"
/// are the same.) All information retrieved is relative
/// to a particular version of the ledger.
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
    limit: Option<u8>,
    destination_account: Option<&'a str>,
    marker: Option<&'a str>,
}

/// This request retrieves a list of currencies that an
/// account can send or receive, based on its trust lines.
/// This is not a thoroughly confirmed list, but it can be
/// used to populate user interfaces.
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
    strict: Option<bool>,
    destination_account: Option<&'a str>,
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
    strict: Option<bool>,
    queue: Option<bool>,
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
    limit: Option<u8>,
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
    deletion_blockers_only: Option<bool>,
    limit: Option<u8>,
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
    limit: Option<u8>,
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
    binary: Option<bool>,
    ledger_index_min: Option<u32>,
    ledger_index_max: Option<u32>,
    forward: Option<bool>,
    limit: Option<u8>,
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
    limit: Option<u8>,
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
