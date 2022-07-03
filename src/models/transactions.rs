//! Transaction models.

use crate::models::*;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Transaction

/// An AccountDelete transaction deletes an account and any objects it
/// owns in the XRP Ledger, if possible, sending the account's remaining
/// XRP to a specified destination account. See Deletion of Accounts for
/// the requirements to delete an account.
///
/// See AccountDelete:
/// `<https://xrpl.org/accountdelete.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct AccountDelete<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::account_delete")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the AccountDelete model.
    ///
    /// See AccountDelete fields:
    /// `<https://xrpl.org/accountdelete.html#accountdelete-fields>`
    destination: &'a str,
    destination_tag: Option<u32>,
}

impl Transaction for AccountDelete<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `AccountDelete` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// An AccountSet transaction modifies the properties of an
/// account in the XRP Ledger.
///
/// See AccountSet:
/// `<https://xrpl.org/accountset.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct AccountSet<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::account_set")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the AccountSet model.
    ///
    /// See AccountSet fields:
    /// `<https://xrpl.org/accountset.html#accountset-fields>`
    clear_flag: Option<u32>,
    domain: Option<&'a str>,
    email_hash: Option<&'a str>,
    message_key: Option<&'a str>,
    set_flag: Option<u32>,
    transfer_rate: Option<u32>,
    tick_size: Option<u32>,
    nftoken_minter: Option<&'a str>,
}

impl Transaction for AccountSet<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `AccountSet` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Cancels an unredeemed Check, removing it from the ledger without
/// sending any money. The source or the destination of the check can
/// cancel a Check at any time using this transaction type. If the Check
/// has expired, any address can cancel it.
///
/// See CheckCancel:
/// `<https://xrpl.org/checkcancel.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct CheckCancel<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::check_cancel")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the CheckCancel model.
    ///
    /// See CheckCancel fields:
    /// `<https://xrpl.org/checkcancel.html#checkcancel-fields>`
    check_id: &'a str,
}

impl Transaction for CheckCancel<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `CheckCancel` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Cancels an unredeemed Check, removing it from the ledger without
/// sending any money. The source or the destination of the check can
/// cancel a Check at any time using this transaction type. If the Check
/// has expired, any address can cancel it.
///
/// See CheckCash:
/// `<https://xrpl.org/checkcash.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct CheckCash<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::check_cash")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the CheckCash model.
    ///
    /// See CheckCash fields:
    /// `<https://xrpl.org/checkcash.html#checkcash-fields>`
    check_id: &'a str,
    amount: Option<Currency>,
    deliver_min: Option<Currency>,
}

impl Transaction for CheckCash<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `CheckCash` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Create a Check object in the ledger, which is a deferred
/// payment that can be cashed by its intended destination.
///
/// See CheckCreate:
/// `<https://xrpl.org/checkcreate.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct CheckCreate<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::check_create")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the CheckCreate model.
    ///
    /// See CheckCreate fields:
    /// `<https://xrpl.org/checkcreate.html#checkcreate-fields>`
    destination: &'a str,
    send_max: Currency,
    destination_tag: Option<u32>,
    expiration: Option<u32>,
    invoice_id: Option<&'a str>,
}

impl Transaction for CheckCreate<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `CheckCreate` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// A DepositPreauth transaction gives another account pre-approval
/// to deliver payments to the sender of this transaction.
///
/// See DepositPreauth:
/// `<https://xrpl.org/depositpreauth.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct DepositPreauth<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::deposit_preauth")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the DepositPreauth model.
    ///
    /// See DepositPreauth fields:
    /// `<https://xrpl.org/depositpreauth.html#depositpreauth-fields>`
    authorize: Option<&'a str>,
    unauthorize: Option<&'a str>,
}

impl Transaction for DepositPreauth<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `DepositPreauth` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Cancels an Escrow and returns escrowed XRP to the sender.
///
/// See EscrowCancel:
/// `<https://xrpl.org/escrowcancel.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct EscrowCancel<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::escrow_cancel")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the EscrowCancel model.
    ///
    /// See EscrowCancel fields:
    /// `<https://xrpl.org/escrowcancel.html#escrowcancel-flags>`
    owner: &'a str,
    offer_sequence: u32,
}

impl Transaction for EscrowCancel<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `EscrowCancel` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Creates an Escrow, which sequests XRP until the escrow process either finishes or is canceled.
///
/// See EscrowCreate:
/// `<https://xrpl.org/escrowcreate.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct EscrowCreate<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::escrow_create")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the EscrowCreate model.
    ///
    /// See EscrowCreate fields:
    /// `<https://xrpl.org/escrowcreate.html#escrowcreate-flags>`
    amount: Currency,
    destination: &'a str,
    destination_tag: Option<&'a str>,
    cancel_after: Option<u32>,
    finish_after: Option<u32>,
    condition: Option<&'a str>,
}

impl Transaction for EscrowCreate<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `EscrowCreate` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Finishes an Escrow and delivers XRP from a held payment to the recipient.
///
/// See EscrowFinish:
/// `<https://xrpl.org/escrowfinish.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct EscrowFinish<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::escrow_finish")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the EscrowFinish model.
    ///
    /// See EscrowFinish fields:
    /// `<https://xrpl.org/escrowfinish.html#escrowfinish-fields>`
    owner: &'a str,
    offer_sequence: u32,
    condition: Option<&'a str>,
    fulfillment: Option<&'a str>,
}

impl Transaction for EscrowFinish<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `EscrowFinish` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Accept offers to buy or sell an NFToken.
///
/// See NFTokenAcceptOffer:
/// `<https://xrpl.org/nftokenacceptoffer.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct NFTokenAcceptOffer<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::nftoken_accept_offer")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the NFTokenAcceptOffer model.
    ///
    /// See NFTokenAcceptOffer fields:
    /// `<https://xrpl.org/nftokenacceptoffer.html#nftokenacceptoffer-fields>`
    nftoken_sell_offer: Option<&'a str>,
    nftoken_buy_offer: Option<&'a str>,
    nftoken_broker_fee: Option<Currency>,
}

impl Transaction for NFTokenAcceptOffer<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `NFTokenAcceptOffer` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Removes a NFToken object from the NFTokenPage in which it is being held,
/// effectively removing the token from the ledger (burning it).
///
/// See NFTokenBurn:
/// `<https://xrpl.org/nftokenburn.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct NFTokenBurn<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::nftoken_burn")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the NFTokenBurn model.
    ///
    /// See NFTokenBurn fields:
    /// `<https://xrpl.org/nftokenburn.html#nftokenburn-fields>`
    nftoken_id: &'a str,
    owner: Option<&'a str>,
}

impl Transaction for NFTokenBurn<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `NFTokenBurn` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Cancels existing token offers created using NFTokenCreateOffer.
///
/// See NFTokenCancelOffer:
/// `<https://xrpl.org/nftokencanceloffer.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct NFTokenCancelOffer<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::nftoken_cancel_offer")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the NFTokenCancelOffer model.
    ///
    /// See NFTokenCancelOffer fields:
    /// `<https://xrpl.org/nftokencanceloffer.html#nftokencanceloffer-fields>`
    /// Lifetime issue
    #[serde(borrow)]
    nftoken_offers: Vec<&'a str>,
}

impl Transaction for NFTokenCancelOffer<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `NFTokenCancelOffer` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Creates either a new Sell offer for an NFToken owned by
/// the account executing the transaction, or a new Buy
/// offer for an NFToken owned by another account.
///
/// See NFTokenCreateOffer:
/// `<https://xrpl.org/nftokencreateoffer.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct NFTokenCreateOffer<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::nftoken_create_offer")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the NFTokenCreateOffer model.
    ///
    /// See NFTokenCreateOffer fields:
    /// `<https://xrpl.org/nftokencreateoffer.html#nftokencreateoffer-fields>`
    nftoken_id: &'a str,
    amount: Currency,
    owner: Option<&'a str>,
    expiration: Option<u32>,
    destination: Option<&'a str>,
}

impl Transaction for NFTokenCreateOffer<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `NFTokenCreateOffer` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// The NFTokenMint transaction creates a non-fungible token and adds it to
/// the relevant NFTokenPage object of the NFTokenMinter as an NFToken object.
///
/// See NFTokenMint:
/// `<https://xrpl.org/nftokenmint.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct NFTokenMint<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::nftoken_mint")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the NFTokenMint model.
    ///
    /// See NFTokenMint fields:
    /// `<https://xrpl.org/nftokenmint.html#nftokenmint-fields>`
    nftoken_taxon: u32,
    issuer: Option<&'a str>,
    transfer_fee: Option<u32>,
    uri: Option<&'a str>,
}

impl Transaction for NFTokenMint<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `NFTokenMint` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Removes an Offer object from the XRP Ledger.
///
/// See OfferCancel:
/// `<https://xrpl.org/offercancel.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct OfferCancel<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::offer_cancel")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the OfferCancel model.
    ///
    /// See OfferCancel fields:
    /// `<https://xrpl.org/offercancel.html#offercancel-fields>`
    offer_sequence: u32,
}

impl Transaction for OfferCancel<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `OfferCancel` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Places an Offer in the decentralized exchange.
///
/// See OfferCreate:
/// `<https://xrpl.org/offercreate.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct OfferCreate<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::offer_create")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the OfferCreate model.
    ///
    /// See OfferCreate fields:
    /// `<https://xrpl.org/offercreate.html#offercreate-fields>`
    taker_gets: Currency,
    taker_pays: Currency,
    expiration: Option<u32>,
    offer_sequence: Option<u32>,
}

impl Transaction for OfferCreate<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `OfferCreate` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Transfers value from one account to another.
///
/// See Payment:
/// `<https://xrpl.org/payment.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct Payment<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::payment")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the Payment model.
    ///
    /// See Payment fields:
    /// `<https://xrpl.org/payment.html#payment-fields>`
    amount: Currency,
    destination: &'a str,
    destination_tag: Option<u32>,
    invoice_id: Option<u32>,
    paths: Option<Vec<Vec<PathStep<'a>>>>,
    send_max: Option<Currency>,
    deliver_min: Option<Currency>,
}

impl Transaction for Payment<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `Payment` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Claim XRP from a payment channel, adjust
/// the payment channel's expiration, or both.
///
/// See PaymentChannelClaim:
/// `<https://xrpl.org/paymentchannelclaim.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct PaymentChannelClaim<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::payment_channel_claim")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the PaymentChannelClaim model.
    ///
    /// See PaymentChannelClaim fields:
    /// `<https://xrpl.org/paymentchannelclaim.html#paymentchannelclaim-fields>`
    channel: &'a str,
    balance: Option<&'a str>,
    amount: Option<&'a str>,
    signature: Option<&'a str>,
    public_key: Option<&'a str>,
}

impl Transaction for PaymentChannelClaim<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `PaymentChannelClaim` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Create a unidirectional channel and fund it with XRP.
///
/// See PaymentChannelCreate fields:
/// `<https://xrpl.org/paymentchannelcreate.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct PaymentChannelCreate<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::payment_channel_create")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the PaymentChannelCreate model.
    ///
    /// See PaymentChannelCreate fields:
    /// `<https://xrpl.org/paymentchannelcreate.html#paymentchannelcreate-fields>`
    amount: Currency,
    destination: &'a str,
    settle_delay: u32,
    public_key: &'a str,
    cancel_after: Option<u32>,
    destination_tag: Option<u32>,
}

impl Transaction for PaymentChannelCreate<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `PaymentChannelCreate` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Add additional XRP to an open payment channel,
/// and optionally update the expiration time of the channel.
///
/// See PaymentChannelFund:
/// `<https://xrpl.org/paymentchannelfund.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct PaymentChannelFund<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::payment_channel_fund")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the PaymentChannelFund model.
    ///
    /// See PaymentChannelFund fields:
    /// `<https://xrpl.org/paymentchannelfund.html#paymentchannelfund-fields>`
    channel: &'a str,
    amount: &'a str,
    expiration: Option<u32>,
}

impl Transaction for PaymentChannelFund<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `PaymentChannelFund` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// You can protect your account by assigning a regular key pair to
/// it and using it instead of the master key pair to sign transactions
/// whenever possible. If your regular key pair is compromised, but
/// your master key pair is not, you can use a SetRegularKey transaction
/// to regain control of your account.
///
/// See SetRegularKey:
/// `<https://xrpl.org/setregularkey.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct SetRegularKey<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::set_regular_key")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the SetRegularKey model.
    ///
    /// See SetRegularKey fields:
    /// `<https://xrpl.org/setregularkey.html#setregularkey-fields>`
    regular_key: Option<&'a str>,
}

impl Transaction for SetRegularKey<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `SetRegularKey` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// The SignerList object type represents a list of parties that,
/// as a group, are authorized to sign a transaction in place of an
/// individual account. You can create, replace, or remove a signer
/// list using a SignerListSet transaction.
///
/// See TicketCreate:
/// `<https://xrpl.org/signerlistset.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct SignerListSet<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::ticket_create")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the TicketCreate model.
    ///
    /// See TicketCreate fields:
    /// `<https://xrpl.org/signerlistset.html#signerlistset-fields>`
    signer_quorum: u32,
}

impl Transaction for SignerListSet<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `TicketCreate` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Sets aside one or more sequence numbers as Tickets.
///
/// See TicketCreate:
/// `<https://xrpl.org/ticketcreate.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct TicketCreate<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::ticket_create")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the TicketCreate model.
    ///
    /// See TicketCreate fields:
    /// `<https://xrpl.org/ticketcreate.html#ticketcreate-fields>`
    ticket_count: u32,
}

impl Transaction for TicketCreate<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `TicketCreate` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Create or modify a trust line linking two accounts.
///
/// See TrustSet:
/// `<https://xrpl.org/trustset.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct TrustSet<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::trust_set")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    last_ledger_sequence: Option<u64>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the TrustSet model.
    ///
    /// See TrustSet fields:
    /// `<https://xrpl.org/trustset.html#trustset-fields>`
    limit_amount: Currency,
    quality_in: Option<u32>,
    quality_out: Option<u32>,
}

impl Transaction for TrustSet<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `TrustSet` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Pseudo-Transactions

/// See EnableAmendment:
/// `<https://xrpl.org/enableamendment.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct EnableAmendment<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::enable_amendment")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    /// The custom fields for the EnableAmendment model.
    ///
    /// See EnableAmendment fields:
    /// `<https://xrpl.org/enableamendment.html#enableamendment-fields>`
    amendment: &'a str,
    ledger_sequence: u32,
}

impl Transaction for EnableAmendment<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `EnableAmendment` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// See SetFee:
/// `<https://xrpl.org/setfee.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct SetFee<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::set_fee")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    /// The custom fields for the SetFee model.
    ///
    /// See SetFee fields:
    /// `<https://xrpl.org/setfee.html#setfee-fields>`
    base_fee: u64,
    reference_fee_units: u32,
    reserve_base: u32,
    reserve_increment: u32,
    ledger_sequence: u32,
}

impl Transaction for SetFee<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `SetFee` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// See UNLModify:
/// `<https://xrpl.org/unlmodify.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct UNLModify<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::unl_modify")]
    transaction_type: TransactionType,
    #[serde(default = "default_account_zero")]
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u64>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    /// The custom fields for the UNLModify model.
    ///
    /// See UNLModify fields:
    /// `<https://xrpl.org/unlmodify.html#unlmodify-fields>`
    ledger_sequence: u16,
    unlmodify_disabling: u8,
    unlmodify_validator: &'a str,
}

impl Transaction for UNLModify<'static> {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect("Unable to serialize `UNLModify` to json.")
    }

    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                flags_int += flag
            }
        }
        flags_int
    }

    fn has_flag(&self) -> bool {
        if self.flags.is_some() && self.iter_to_int() > 0 {
            return true;
        }
        false
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}
