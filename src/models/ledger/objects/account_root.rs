use crate::models::ledger::LedgerEntryType;
use crate::models::{Amount, Model};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;
use strum_macros::{AsRefStr, Display, EnumIter};

/// There are several options which can be either enabled or disabled for an account.
/// These options can be changed with an `AccountSet` transaction.
///
/// See `AccountRoot` flags:
/// `<https://xrpl.org/accountroot.html#accountroot-flags>`
#[derive(
    Debug, Eq, PartialEq, Clone, Serialize_repr, Deserialize_repr, Display, AsRefStr, EnumIter,
)]
#[repr(u32)]
pub enum AccountRootFlag {
    /// This account is an Automated Market Maker instance.
    LsfAmm = 0x02000000,
    /// Enable rippling on this addresses's trust lines by default.
    /// Required for issuing addresses; discouraged for others.
    LsfDefaultRipple = 0x00800000,
    /// This account can only receive funds from transactions it sends, and from preauthorized
    /// accounts. (It has `DepositAuth` enabled.)
    LsfDepositAuth = 0x01000000,
    /// Disallows use of the master key to sign transactions for this account.
    LsfDisableMaster = 0x00100000,
    /// Client applications should not send XRP to this account. Not enforced by rippled.
    LsfDisallowXRP = 0x00080000,
    /// All assets issued by this address are frozen.
    LsfGlobalFreeze = 0x00400000,
    /// This address cannot freeze trust lines connected to it. Once enabled, cannot be disabled.
    LsfNoFreeze = 0x00200000,
    /// The account has used its free SetRegularKey transaction.
    LsfPasswordSpent = 0x00010000,
    /// This account must individually approve other users for those users to hold this account's
    /// tokens.
    LsfRequireAuth = 0x00040000,
    /// Requires incoming payments to specify a Destination Tag.
    LsfRequireDestTag = 0x00020000,
}

/// The `AccountRoot` object type describes a single account, its settings, and XRP balance.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AccountRoot<'a> {
    /// The identifying (classic) address of this account.
    account: &'a str,
    /// A bit-map of boolean flags enabled for this account.
    flags: Vec<AccountRootFlag>,
    /// The value 0x0061, mapped to the string AccountRoot, indicates that this is an AccountRoot
    /// object.
    ledger_entry_type: LedgerEntryType,
    /// The number of objects this account owns in the ledger, which contributes to its owner
    /// reserve.
    owner_count: u32,
    /// The identifying hash of the transaction that most recently modified this object.
    previous_txn_id: &'a str,
    /// The index of the ledger that contains the transaction that most recently modified this object.
    previous_txn_lgr_seq: u32,
    /// The sequence number of the next valid transaction for this account.
    sequence: u32,
    /// The identifying hash of the transaction most recently sent by this account. This field must
    /// be enabled to use the `AccountTxnID` transaction field. To enable it, send an `AccountSet`
    /// transaction with the `asfAccountTxnID` flag enabled.
    #[serde(rename = "AccountTxnID")]
    account_txn_id: Option<&'a str>,
    /// The account's current XRP balance in drops, represented as a string.
    balance: Option<Amount>,
    /// How many total of this account's issued non-fungible tokens have been burned. This number
    /// is always equal or less than `MintedNFTokens`.
    #[serde(rename = "BurnedNFTokens")]
    burned_nftokens: Option<u32>,
    /// A domain associated with this account. In JSON, this is the hexadecimal for the ASCII
    /// representation of the domain. Cannot be more than 256 bytes in length.
    domain: Option<&'a str>,
    /// The md5 hash of an email address. Clients can use this to look up an avatar through services
    /// such as Gravatar
    email_hash: Option<&'a str>,
    /// A public key that may be used to send encrypted messages to this account. In JSON, uses
    /// hexadecimal. Must be exactly 33 bytes, with the first byte indicating the key type: 0x02 or
    /// 0x03 for secp256k1 keys, 0xED for Ed25519 keys.
    message_key: Option<&'a str>,
    /// How many total non-fungible tokens have been minted by and on behalf of this account.
    #[serde(rename = "MintedNFTokens")]
    minted_nftokens: Option<u32>,
    /// Another account that can mint non-fungible tokens on behalf of this account.
    #[serde(rename = "NFTokenMinter")]
    nftoken_minter: Option<&'a str>,
    /// The address of a key pair that can be used to sign transactions for this account instead of
    /// the master key. Use a SetRegularKey transaction to change this value.
    regular_key: Option<&'a str>,
    /// How many `Tickets` this account owns in the ledger. This is updated automatically to ensure
    /// that the account stays within the hard limit of 250 Tickets at a time. This field is omitted
    /// if the account has zero Tickets.
    ticket_count: Option<u8>,
    /// How many significant digits to use for exchange rates of Offers involving currencies issued
    /// by this address. Valid values are 3 to 15, inclusive.
    tick_size: Option<u8>,
    /// A transfer fee to charge other users for sending currency issued by this account to each other.
    transfer_rate: Option<u32>,
    /// An arbitrary 256-bit value that users can set.
    wallet_locator: Option<&'a str>,
    /// Unused. (The code supports this field but there is no way to set it.)
    wallet_size: Option<u32>,
}

impl<'a> Model for AccountRoot<'a> {}

impl<'a> AccountRoot<'a> {
    fn new(
        account: &'a str,
        flags: Vec<AccountRootFlag>,
        owner_count: u32,
        previous_txn_id: &'a str,
        previous_txn_lgr_seq: u32,
        sequence: u32,
        account_txn_id: Option<&'a str>,
        balance: Option<Amount>,
        burned_nftokens: Option<u32>,
        domain: Option<&'a str>,
        email_hash: Option<&'a str>,
        message_key: Option<&'a str>,
        minted_nftokens: Option<u32>,
        nftoken_minter: Option<&'a str>,
        regular_key: Option<&'a str>,
        ticket_count: Option<u8>,
        tick_size: Option<u8>,
        transfer_rate: Option<u32>,
        wallet_locator: Option<&'a str>,
        wallet_size: Option<u32>,
    ) -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::AccountRoot,
            account,
            flags,
            owner_count,
            previous_txn_id,
            previous_txn_lgr_seq,
            sequence,
            account_txn_id,
            balance,
            burned_nftokens,
            domain,
            email_hash,
            message_key,
            minted_nftokens,
            nftoken_minter,
            regular_key,
            ticket_count,
            tick_size,
            transfer_rate,
            wallet_locator,
            wallet_size,
        }
    }
}