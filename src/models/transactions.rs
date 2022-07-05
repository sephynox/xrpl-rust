//! Transaction models.

use crate::{
    constants::{
        DISABLE_TICK_SIZE, MAX_DOMAIN_LENGTH, MAX_TICK_SIZE, MAX_TRANSFER_RATE, MIN_TICK_SIZE,
        MIN_TRANSFER_RATE, SPECIAL_CASE_TRANFER_RATE,
    },
    models::*,
};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use base::Base;

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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
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

impl Base for AccountDelete<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `AccountDelete` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for AccountDelete<'static> {
    fn iter_to_int(&self) -> u32 {
        0
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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<AccountSetFlag>>,
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

impl Base for AccountSet<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json_str = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.").as_str();
    //     transaction_json_str
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `AccountSet` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    #[doc(hidden)]
    fn get_errors(&self) -> Vec<&str> {
        let mut errors: Vec<&str> = Vec::new();
        let tick_size = self.get_tick_size_error();
        let transfer_rate = self.get_transfer_rate_error();
        let domain = self.get_domain_error();
        let clear_flag = self.get_clear_flag_error();
        let nftoken_minter = self.get_nftoken_minter_error();
        if let Some(value) = tick_size {
            errors.push(value);
        }
        if let Some(value) = transfer_rate {
            errors.push(value);
        }
        if let Some(value) = domain {
            errors.push(value);
        }
        if let Some(value) = clear_flag {
            errors.push(value);
        }
        if let Some(value) = nftoken_minter {
            errors.push(value);
        }
        errors
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for AccountSet<'static> {
    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                match flag {
                    AccountSetFlag::AsfAccountTxnID => flags_int += 0x00000005,
                    AccountSetFlag::AsfAuthorizedNFTokenMinter => flags_int += 0x0000000A,
                    AccountSetFlag::AsfDefaultRipple => flags_int += 0x00000008,
                    AccountSetFlag::AsfDepositAuth => flags_int += 0x00000009,
                    AccountSetFlag::AsfDisableMaster => flags_int += 0x00000004,
                    AccountSetFlag::AsfDisallowXRP => flags_int += 0x00000003,
                    AccountSetFlag::AsfGlobalFreeze => flags_int += 0x00000007,
                    AccountSetFlag::AsfNoFreeze => flags_int += 0x00000006,
                    AccountSetFlag::AsfRequireAuth => flags_int += 0x00000002,
                    AccountSetFlag::AsfRequireDest => flags_int += 0x00000001,
                }
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

impl AccountSetError for AccountSet<'static> {
    fn get_tick_size_error(&self) -> Option<&str> {
        if self.tick_size.is_some() {
            let tick_size = self.tick_size.unwrap();
            if tick_size > MAX_TICK_SIZE {
                return Some("`tick_size` is above 15");
            } else if tick_size < MIN_TICK_SIZE && tick_size != DISABLE_TICK_SIZE {
                return Some("`tick_size` is below 3.");
            }
        }
        None
    }

    fn get_transfer_rate_error(&self) -> Option<&str> {
        let transafer_rate = &self.transfer_rate.unwrap();
        if self.transfer_rate.is_some() && transafer_rate > &MAX_TRANSFER_RATE {
            return Some("`transafer_rate` is above 2000000000.");
        }
        if self.transfer_rate.is_some()
            && transafer_rate < &MIN_TRANSFER_RATE
            && transafer_rate != &SPECIAL_CASE_TRANFER_RATE
        {
            return Some("`transafer_rate` is below 1000000000.");
        }
        None
    }

    fn get_domain_error(&self) -> Option<&str> {
        if self.domain.is_some()
            && self.domain.unwrap().to_lowercase().as_str() != self.domain.unwrap()
        {
            return Some("`domain` is not lowercase");
        }
        if self.domain.is_some() && self.domain.unwrap().len() > MAX_DOMAIN_LENGTH {
            return Some("`domain` must not be longer than 256 characters");
        }
        None
    }

    fn get_clear_flag_error(&self) -> Option<&str> {
        if self.clear_flag.is_some() && self.clear_flag == self.set_flag {
            return Some("`clear_flag` must not be equal to the `set_flag`");
        }
        None
    }

    fn get_nftoken_minter_error(&self) -> Option<&str> {
        // TODO: `set_flag` and `clear_flag` should be typed as `AccountSetFlag`.
        // if self.nftoken_minter.is_some() && self.set_flag.unwrap() != AccountSetFlag::AsfAuthorizedNFTokenMinter {
        //     return Some("Will not set the minter unless AccountSetFlag.ASF_AUTHORIZED_NFTOKEN_MINTER is set.");
        // }
        // if self.nftoken_minter.is_none() && self.set_flag.unwrap() == AccountSetFlag::AsfAuthorizedNFTokenMinter {
        //     return Some("`nftoken_minter` must be present if `AccountSetFlag.AsfAuthorizedNFTokenMinter` is set.");
        // }
        // if self.nftoken_minter.is_some() && self.clear_flag.unwrap() == AccountSetFlag::AsfAuthorizedNFTokenMinter {
        //     return Some("`nftoken_minter` must not be present if AccountSetFlag.AsfAuthorizedNFTokenMinter is unset using `clear_flag`")
        // }
        None
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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
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

impl Base for CheckCancel<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `CheckCancel` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for CheckCancel<'static> {
    fn iter_to_int(&self) -> u32 {
        0
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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
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

impl Base for CheckCash<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `CheckCash` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for CheckCash<'static> {
    fn iter_to_int(&self) -> u32 {
        0
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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
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

impl Base for CheckCreate<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `CheckCreate` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for CheckCreate<'static> {
    fn iter_to_int(&self) -> u32 {
        0
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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
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

impl Base for DepositPreauth<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `DepositPreauth` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for DepositPreauth<'static> {
    fn iter_to_int(&self) -> u32 {
        0
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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
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

impl Base for EscrowCancel<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `EscrowCancel` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for EscrowCancel<'static> {
    fn iter_to_int(&self) -> u32 {
        0
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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
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

impl Base for EscrowCreate<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `EscrowCreate` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for EscrowCreate<'static> {
    fn iter_to_int(&self) -> u32 {
        0
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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
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

impl Base for EscrowFinish<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `EscrowFinish` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for EscrowFinish<'static> {
    fn iter_to_int(&self) -> u32 {
        0
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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
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

impl Base for NFTokenAcceptOffer<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `NFTokenAcceptOffer` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for NFTokenAcceptOffer<'static> {
    fn iter_to_int(&self) -> u32 {
        0
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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
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

impl Base for NFTokenBurn<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `NFTokenBurn` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for NFTokenBurn<'static> {
    fn iter_to_int(&self) -> u32 {
        0
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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
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

impl Base for NFTokenCancelOffer<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `NFTokenCancelOffer` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for NFTokenCancelOffer<'static> {
    fn iter_to_int(&self) -> u32 {
        0
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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<NFTokenCreateOfferFlag>>,
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

impl Base for NFTokenCreateOffer<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `NFTokenCreateOffer` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for NFTokenCreateOffer<'static> {
    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                match flag {
                    &NFTokenCreateOfferFlag::TfSellOffer => flags_int += 0x00000001,
                }
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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<NFTokenMintFlag>>,
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

impl Base for NFTokenMint<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `NFTokenMint` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for NFTokenMint<'static> {
    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                match flag {
                    NFTokenMintFlag::TfBurnable => flags_int += 0x00000001,
                    NFTokenMintFlag::TfOnlyXRP => flags_int += 0x00000002,
                    NFTokenMintFlag::TfTrustline => flags_int += 0x00000004,
                    NFTokenMintFlag::TfTransferable => flags_int += 0x00000008,
                }
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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
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

impl Base for OfferCancel<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `OfferCancel` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for OfferCancel<'static> {
    fn iter_to_int(&self) -> u32 {
        0
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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<OfferCreateFlag>>,
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

impl Base for OfferCreate<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `OfferCreate` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for OfferCreate<'static> {
    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                match flag {
                    OfferCreateFlag::TfPassive => flags_int += 0x00010000,
                    OfferCreateFlag::TfImmediateOrCancel => flags_int += 0x00020000,
                    OfferCreateFlag::TfFillOrKill => flags_int += 0x00040000,
                    OfferCreateFlag::TfSell => flags_int += 0x00080000,
                }
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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<PaymentFlag>>,
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

impl Base for Payment<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `Payment` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for Payment<'static> {
    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                match flag {
                    PaymentFlag::TfNoDirectRipple => flags_int += 0x00010000,
                    PaymentFlag::TfPartialPayment => flags_int += 0x00020000,
                    PaymentFlag::TfLimitQuality => flags_int += 0x00040000,
                }
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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<PaymentChannelClaimFlag>>,
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

impl Base for PaymentChannelClaim<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json = serde_json::to_value(&self)
            .expect("Unable to serialize `PaymentChannelClaim` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for PaymentChannelClaim<'static> {
    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                match flag {
                    PaymentChannelClaimFlag::TfRenew => flags_int += 0x00010000,
                    PaymentChannelClaimFlag::TfClose => flags_int += 0x00020000,
                }
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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
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

impl Base for PaymentChannelCreate<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json = serde_json::to_value(&self)
            .expect("Unable to serialize `PaymentChannelCreate` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for PaymentChannelCreate<'static> {
    fn iter_to_int(&self) -> u32 {
        0
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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
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

impl Base for PaymentChannelFund<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `PaymentChannelFund` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for PaymentChannelFund<'static> {
    fn iter_to_int(&self) -> u32 {
        0
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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
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

impl Base for SetRegularKey<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `SetRegularKey` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for SetRegularKey<'static> {
    fn iter_to_int(&self) -> u32 {
        0
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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
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

impl Base for SignerListSet<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `SignerListSet` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for SignerListSet<'static> {
    fn iter_to_int(&self) -> u32 {
        0
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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
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

impl Base for TicketCreate<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `TicketCreate` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for TicketCreate<'static> {
    fn iter_to_int(&self) -> u32 {
        0
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
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<TrustSetFlag>>,
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

impl Base for TrustSet<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `TrustSet` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for TrustSet<'static> {
    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                match flag {
                    TrustSetFlag::TfSetAuth => flags_int += 0x00010000,
                    TrustSetFlag::TfSetNoRipple => flags_int += 0x00020000,
                    TrustSetFlag::TfClearNoRipple => flags_int += 0x00040000,
                    TrustSetFlag::TfSetFreeze => flags_int += 0x00100000,
                    TrustSetFlag::TfClearFreeze => flags_int += 0x00200000,
                }
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
    sequence: Option<u32>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<EnableAmendmentFlag>>,
    /// The custom fields for the EnableAmendment model.
    ///
    /// See EnableAmendment fields:
    /// `<https://xrpl.org/enableamendment.html#enableamendment-fields>`
    amendment: &'a str,
    ledger_sequence: u32,
}

impl Base for EnableAmendment<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `EnableAmendment` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for EnableAmendment<'static> {
    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                match flag {
                    EnableAmendmentFlag::TfGotMajority => flags_int += 0x00010000,
                    EnableAmendmentFlag::TfLostMajority => flags_int += 0x00020000,
                }
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
    sequence: Option<u32>,
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

impl Base for SetFee<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `SetFee` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for SetFee<'static> {
    fn iter_to_int(&self) -> u32 {
        0
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
    sequence: Option<u32>,
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

impl Base for UNLModify<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `UNLModify` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }

    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

impl Transaction for UNLModify<'static> {
    fn iter_to_int(&self) -> u32 {
        0
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

#[cfg(test)]
mod test {
    use alloc::vec;

    use super::*;

    #[test]
    fn test_to_json() {
        let sequence: u32 = 1;
        let last_ledger_sequence: u32 = 72779837;
        let flags = vec![OfferCreateFlag::TfImmediateOrCancel];
        let xrp_amount = "1000000";
        let usd_amount = "0.3";
        let offer_create: OfferCreate = OfferCreate {
            transaction_type: TransactionType::OfferCreate,
            account: "rpXhhWmCvDwkzNtRbm7mmD1vZqdfatQNEe",
            fee: Some("10"),
            sequence: Some(sequence),
            last_ledger_sequence: Some(last_ledger_sequence),
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: Some(flags),
            memos: None,
            signers: None,
            taker_gets: Currency::Xrp {
                amount: Some(Borrowed(xrp_amount)),
                currency: Borrowed("XRP"),
            },
            taker_pays: Currency::IssuedCurrency {
                amount: Some(Borrowed(usd_amount)),
                currency: Borrowed("USD"),
                issuer: Borrowed("rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq"),
            },
            expiration: None,
            offer_sequence: None,
        };
        let actual = offer_create.to_json_value();
        let json = r#"{"Account":"rpXhhWmCvDwkzNtRbm7mmD1vZqdfatQNEe","Fee":"10","Sequence":1,"LastLedgerSequence":72779837,"Flags":131072,"TakerGets":{"amount":"1000000","currency":"XRP"},"TakerPays":{"amount":"0.3","currency":"USD","issuer":"rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq"}}"#;
        let expect: Value = serde_json::from_str(json).unwrap();
        assert_eq!(actual, expect)
    }

    #[test]
    fn test_has_flag() {
        let sequence: u32 = 1;
        let last_ledger_sequence: u32 = 72779837;
        let flags = vec![OfferCreateFlag::TfImmediateOrCancel];
        let xrp_amount = "1000000";
        let usd_amount = "0.3";
        let offer_create: OfferCreate = OfferCreate {
            transaction_type: TransactionType::OfferCreate,
            account: "rpXhhWmCvDwkzNtRbm7mmD1vZqdfatQNEe",
            fee: Some("10"),
            sequence: Some(sequence),
            last_ledger_sequence: Some(last_ledger_sequence),
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: Some(flags),
            memos: None,
            signers: None,
            taker_gets: Currency::Xrp {
                amount: Some(Borrowed(xrp_amount)),
                currency: Borrowed("XRP"),
            },
            taker_pays: Currency::IssuedCurrency {
                amount: Some(Borrowed(usd_amount)),
                currency: Borrowed("USD"),
                issuer: Borrowed("rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq"),
            },
            expiration: None,
            offer_sequence: None,
        };
        assert!(offer_create.has_flag())
    }

    #[test]
    fn test_get_transaction_type() {
        let sequence: u32 = 1;
        let last_ledger_sequence: u32 = 72779837;
        let flags = vec![OfferCreateFlag::TfImmediateOrCancel];
        let xrp_amount = "1000000";
        let usd_amount = "0.3";
        let offer_create: OfferCreate = OfferCreate {
            transaction_type: TransactionType::OfferCreate,
            account: "rpXhhWmCvDwkzNtRbm7mmD1vZqdfatQNEe",
            fee: Some("10"),
            sequence: Some(sequence),
            last_ledger_sequence: Some(last_ledger_sequence),
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: Some(flags),
            memos: None,
            signers: None,
            taker_gets: Currency::Xrp {
                amount: Some(Borrowed(xrp_amount)),
                currency: Borrowed("XRP"),
            },
            taker_pays: Currency::IssuedCurrency {
                amount: Some(Borrowed(usd_amount)),
                currency: Borrowed("USD"),
                issuer: Borrowed("rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq"),
            },
            expiration: None,
            offer_sequence: None,
        };
        let actual = offer_create.get_transaction_type();
        let expect = TransactionType::OfferCreate;
        assert_eq!(actual, expect)
    }
}
