//! Transaction models.

use crate::{
    constants::{
        DISABLE_TICK_SIZE, MAX_DOMAIN_LENGTH, MAX_TICK_SIZE, MAX_TRANSFER_FEE, MAX_TRANSFER_RATE,
        MAX_URI_LENGTH, MIN_TICK_SIZE, MIN_TRANSFER_RATE, SPECIAL_CASE_TRANFER_RATE,
    },
    models::*,
};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use model::Model;

use super::exceptions::{
    CheckCashException, DepositPreauthException, EscrowCreateException, EscrowFinishException,
    NFTokenAcceptOfferException, XRPLModelException, XRPLTransactionException,
};

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

impl Model for AccountDelete<'static> {
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
}

impl Transaction for AccountDelete<'static> {
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
    clear_flag: Option<AccountSetFlag>,
    domain: Option<&'a str>,
    email_hash: Option<&'a str>,
    message_key: Option<&'a str>,
    set_flag: Option<AccountSetFlag>,
    transfer_rate: Option<u32>,
    tick_size: Option<u32>,
    nftoken_minter: Option<&'a str>,
}

impl Model for AccountSet<'static> {
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

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_tick_size_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::AccountSetError(error),
            )),
            Ok(_no_error) => match self.get_transfer_rate_error() {
                Err(error) => Err(XRPLModelException::XRPLTransactionError(
                    XRPLTransactionException::AccountSetError(error),
                )),
                Ok(_no_error) => match self.get_domain_error() {
                    Err(error) => Err(XRPLModelException::XRPLTransactionError(
                        XRPLTransactionException::AccountSetError(error),
                    )),
                    Ok(_no_error) => match self.get_clear_flag_error() {
                        Err(error) => Err(XRPLModelException::XRPLTransactionError(
                            XRPLTransactionException::AccountSetError(error),
                        )),
                        Ok(_no_error) => match self.get_nftoken_minter_error() {
                            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                                XRPLTransactionException::AccountSetError(error),
                            )),
                            Ok(_no_error) => Ok(()),
                        },
                    },
                },
            },
        }
    }
}

impl Transaction for AccountSet<'static> {
    fn iter_to_int(&self) -> u32 {
        if self.set_flag.is_some() {
            match self.set_flag.as_ref().unwrap() {
                AccountSetFlag::AsfAccountTxnID => 0x00000005,
                AccountSetFlag::AsfAuthorizedNFTokenMinter => 0x0000000A,
                AccountSetFlag::AsfDefaultRipple => 0x00000008,
                AccountSetFlag::AsfDepositAuth => 0x00000009,
                AccountSetFlag::AsfDisableMaster => 0x00000004,
                AccountSetFlag::AsfDisallowXRP => 0x00000003,
                AccountSetFlag::AsfGlobalFreeze => 0x00000007,
                AccountSetFlag::AsfNoFreeze => 0x00000006,
                AccountSetFlag::AsfRequireAuth => 0x00000002,
                AccountSetFlag::AsfRequireDest => 0x00000001,
            }
        } else if self.clear_flag.is_some() {
            match self.clear_flag.as_ref().unwrap() {
                AccountSetFlag::AsfAccountTxnID => 0x00000005,
                AccountSetFlag::AsfAuthorizedNFTokenMinter => 0x0000000A,
                AccountSetFlag::AsfDefaultRipple => 0x00000008,
                AccountSetFlag::AsfDepositAuth => 0x00000009,
                AccountSetFlag::AsfDisableMaster => 0x00000004,
                AccountSetFlag::AsfDisallowXRP => 0x00000003,
                AccountSetFlag::AsfGlobalFreeze => 0x00000007,
                AccountSetFlag::AsfNoFreeze => 0x00000006,
                AccountSetFlag::AsfRequireAuth => 0x00000002,
                AccountSetFlag::AsfRequireDest => 0x00000001,
            }
        } else {
            0
        }
    }

    fn has_flag(&self, flag: Flag) -> bool {
        let mut has_flag = false;
        if self.iter_to_int() > 0 {
            match flag {
                Flag::AccountSet(account_set_flag) => {
                    match account_set_flag {
                        AccountSetFlag::AsfAccountTxnID => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&AccountSetFlag::AsfAccountTxnID)
                            {
                                has_flag = true
                            };
                        }
                        AccountSetFlag::AsfAuthorizedNFTokenMinter => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&AccountSetFlag::AsfAuthorizedNFTokenMinter)
                            {
                                has_flag = true
                            };
                        }
                        AccountSetFlag::AsfDefaultRipple => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&AccountSetFlag::AsfDefaultRipple)
                            {
                                has_flag = true
                            };
                        }
                        AccountSetFlag::AsfDepositAuth => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&AccountSetFlag::AsfDepositAuth)
                            {
                                has_flag = true
                            };
                        }
                        AccountSetFlag::AsfDisableMaster => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&AccountSetFlag::AsfDisableMaster)
                            {
                                has_flag = true
                            };
                        }
                        AccountSetFlag::AsfDisallowXRP => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&AccountSetFlag::AsfDisallowXRP)
                            {
                                has_flag = true
                            };
                        }
                        AccountSetFlag::AsfGlobalFreeze => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&AccountSetFlag::AsfGlobalFreeze)
                            {
                                has_flag = true
                            };
                        }
                        AccountSetFlag::AsfNoFreeze => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&AccountSetFlag::AsfNoFreeze)
                            {
                                has_flag = true
                            };
                        }
                        AccountSetFlag::AsfRequireAuth => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&AccountSetFlag::AsfRequireAuth)
                            {
                                has_flag = true
                            };
                        }
                        AccountSetFlag::AsfRequireDest => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&AccountSetFlag::AsfRequireDest)
                            {
                                has_flag = true
                            };
                        }
                    };
                }
                _ => has_flag = false,
            }
        }
        has_flag
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl AccountSetError for AccountSet<'static> {
    fn get_tick_size_error(&self) -> Result<(), AccountSetException> {
        match self.tick_size {
            Some(tick_size) => match tick_size > MAX_TICK_SIZE {
                true => Err(AccountSetException::InvalidTickSizeTooHigh {
                    max: 15,
                    found: tick_size,
                }),
                false => match tick_size < MIN_TICK_SIZE && tick_size != DISABLE_TICK_SIZE {
                    true => Err(AccountSetException::InvalidTickSizeTooLow {
                        min: 3,
                        found: tick_size,
                    }),
                    false => Ok(()),
                },
            },
            None => Ok(()),
        }
    }

    fn get_transfer_rate_error(&self) -> Result<(), AccountSetException> {
        match self.transfer_rate {
            Some(transfer_rate) => match transfer_rate > MAX_TRANSFER_RATE {
                true => Err(AccountSetException::InvalidTransferRateTooHigh {
                    max: MAX_TRANSFER_RATE,
                    found: transfer_rate,
                }),
                false => match transfer_rate < MIN_TRANSFER_RATE
                    && transfer_rate != SPECIAL_CASE_TRANFER_RATE
                {
                    true => Err(AccountSetException::InvalidTransferRateTooLow {
                        min: MIN_TRANSFER_RATE,
                        found: transfer_rate,
                    }),
                    false => Ok(()),
                },
            },
            None => Ok(()),
        }
    }

    fn get_domain_error(&self) -> Result<(), AccountSetException> {
        match self.domain {
            Some(domain) => match domain.to_lowercase().as_str() != domain {
                true => Err(AccountSetException::InvalidDomainIsNotLowercase),
                false => match domain.len() > MAX_DOMAIN_LENGTH {
                    true => Err(AccountSetException::InvalidDomainTooLong {
                        max: MAX_DOMAIN_LENGTH,
                        found: domain.len(),
                    }),
                    false => Ok(()),
                },
            },
            None => Ok(()),
        }
    }

    fn get_clear_flag_error(&self) -> Result<(), AccountSetException> {
        match self.clear_flag.as_ref() {
            Some(_clear_flag) => match self.clear_flag == self.set_flag {
                true => Err(AccountSetException::InvalidClearFlagMustNotEqualSetFlag),
                false => Ok(()),
            },
            None => Ok(()),
        }
    }

    fn get_nftoken_minter_error(&self) -> Result<(), AccountSetException> {
        match self.nftoken_minter {
            Some(_nftoken_minter) => match self.set_flag.as_ref() {
                Some(_set_flag) => Ok(()),
                None => match self.clear_flag.as_ref() {
                    Some(clear_flag) => match clear_flag {
                        AccountSetFlag::AsfAuthorizedNFTokenMinter => Err(AccountSetException::InvalidNftokenMinterMustNotBeSetIfAsfAuthorizedNftokenMinterIsUnset),
                        _ => Ok(()),
                    }
                    None => Err(AccountSetException::InvalidMustSetAsfAuthorizedNftokenMinterFlagToSetMinter),
                },
            },
            None => match self.set_flag.as_ref() {
                Some(_set_flag) => Err(AccountSetException::InvalidNftokenMinterMustBeSetIfAsfAuthorizedNftokenMinterIsSet),
                None => Ok(()),
            }
        }
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

impl Model for CheckCancel<'static> {
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
}

impl Transaction for CheckCancel<'static> {
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

impl Model for CheckCash<'static> {
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

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_amount_and_deliver_min_error() {
            Ok(_no_error) => Ok(()),
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::CheckCashError(error),
            )),
        }
    }
}

impl Transaction for CheckCash<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl CheckCashError for CheckCash<'static> {
    fn get_amount_and_deliver_min_error(&self) -> Result<(), CheckCashException> {
        match self.amount.is_none() && self.deliver_min.is_none() {
            true => Err(CheckCashException::InvalidMustSetAmountOrDeliverMin),
            false => match self.amount.is_some() && self.deliver_min.is_some() {
                true => Err(CheckCashException::InvalidMustNotSetAmountAndDeliverMin),
                false => Ok(()),
            },
        }
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

impl Model for CheckCreate<'static> {
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
}

impl Transaction for CheckCreate<'static> {
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

impl Model for DepositPreauth<'static> {
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

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_authorize_and_unauthorize_error() {
            Ok(_no_error) => Ok(()),
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::DepositPreauthError(error),
            )),
        }
    }
}

impl Transaction for DepositPreauth<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl DepositPreauthError for DepositPreauth<'static> {
    fn get_authorize_and_unauthorize_error(&self) -> Result<(), DepositPreauthException> {
        match self.authorize.is_none() && self.unauthorize.is_none() {
            true => Err(DepositPreauthException::InvalidMustSetAuthorizeOrUnauthorize),
            false => match self.authorize.is_some() && self.unauthorize.is_some() {
                true => Err(DepositPreauthException::InvalidMustNotSetAuthorizeAndUnauthorize),
                false => Ok(()),
            },
        }
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

impl Model for EscrowCancel<'static> {
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
}

impl Transaction for EscrowCancel<'static> {
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

impl Model for EscrowCreate<'static> {
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

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_finish_after_error() {
            Ok(_no_error) => Ok(()),
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::EscrowCreateError(error),
            )),
        }
    }
}

impl Transaction for EscrowCreate<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl EscrowCreateError for EscrowCreate<'static> {
    fn get_finish_after_error(&self) -> Result<(), EscrowCreateException> {
        match self.finish_after {
            Some(finish_after) => match self.cancel_after {
                Some(cancel_after) => match finish_after >= cancel_after {
                    true => {
                        Err(EscrowCreateException::InvalidCancelAfterMustNotBeBeforeFinishAfter)
                    }
                    false => Ok(()),
                },
                None => Ok(()),
            },
            None => Ok(()),
        }
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

impl Model for EscrowFinish<'static> {
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

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_condition_and_fulfillment_error() {
            Ok(_no_error) => Ok(()),
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::EscrowFinishError(error),
            )),
        }
    }
}

impl Transaction for EscrowFinish<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl EscrowFinishError for EscrowFinish<'static> {
    fn get_condition_and_fulfillment_error(&self) -> Result<(), EscrowFinishException> {
        match (self.condition.is_some() && self.fulfillment.is_none())
            || (self.condition.is_none() && self.condition.is_some())
        {
            true => Err(EscrowFinishException::InvalidIfOneSetBothConditionAndFulfillmentMustBeSet),
            false => Ok(()),
        }
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

impl Model for NFTokenAcceptOffer<'static> {
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

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_brokered_mode_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::NFTokenAcceptOfferError(error),
            )),
            Ok(_no_error) => match self.get_nftoken_broker_fee_error() {
                Err(error) => Err(XRPLModelException::XRPLTransactionError(
                    XRPLTransactionException::NFTokenAcceptOfferError(error),
                )),
                Ok(_no_error) => Ok(()),
            },
        }
    }
}

impl Transaction for NFTokenAcceptOffer<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl NFTokenAcceptOfferError for NFTokenAcceptOffer<'static> {
    fn get_brokered_mode_error(&self) -> Result<(), NFTokenAcceptOfferException> {
        match self.nftoken_broker_fee.as_ref() {
            Some(_nftoken_broker_fee) => match self.nftoken_sell_offer.is_none() && self.nftoken_buy_offer.is_none() {
                true => Err(NFTokenAcceptOfferException::InvalidMustSetEitherNftokenBuyOfferOrNftokenSellOffer),
                false => Ok(()),
            }
            None => Ok(()),
        }
    }
    fn get_nftoken_broker_fee_error(&self) -> Result<(), NFTokenAcceptOfferException> {
        match self.nftoken_broker_fee.as_ref() {
            Some(nftoken_broker_fee) => match nftoken_broker_fee.get_value_as_u32() == 0 {
                true => Err(NFTokenAcceptOfferException::InvalidBrokerFeeMustBeGreaterZero),
                false => Ok(()),
            },
            None => Ok(()),
        }
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

impl Model for NFTokenBurn<'static> {
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
}

impl Transaction for NFTokenBurn<'static> {
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

impl Model for NFTokenCancelOffer<'static> {
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

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_nftoken_offers_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::NFTokenCancelOfferError(error),
            )),
            Ok(_no_error) => Ok(()),
        }
    }
}

impl Transaction for NFTokenCancelOffer<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl NFTokenCancelOfferError for NFTokenCancelOffer<'static> {
    fn get_nftoken_offers_error(&self) -> Result<(), NFTokenCancelOfferException> {
        match self.nftoken_offers.is_empty() {
            true => Err(NFTokenCancelOfferException::InvalidMustIncludeOneNFTokenOffer),
            false => Ok(()),
        }
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

impl Model for NFTokenCreateOffer<'static> {
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

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_amount_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::NFTokenCreateOfferError(error),
            )),
            Ok(_no_error) => match self.get_destination_error() {
                Err(error) => Err(XRPLModelException::XRPLTransactionError(
                    XRPLTransactionException::NFTokenCreateOfferError(error),
                )),
                Ok(_no_error) => match self.get_owner_error() {
                    Err(error) => Err(XRPLModelException::XRPLTransactionError(
                        XRPLTransactionException::NFTokenCreateOfferError(error),
                    )),
                    Ok(_no_error) => Ok(()),
                },
            },
        }
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

    fn has_flag(&self, flag: Flag) -> bool {
        let mut has_flag = false;
        if self.iter_to_int() > 0 {
            match flag {
                Flag::NFTokenCreateOffer(nftoken_create_offer_flag) => {
                    match nftoken_create_offer_flag {
                        NFTokenCreateOfferFlag::TfSellOffer => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&NFTokenCreateOfferFlag::TfSellOffer)
                            {
                                has_flag = true
                            };
                        }
                    }
                }
                _ => has_flag = false,
            };
        }
        has_flag
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl NFTokenCreateOfferError for NFTokenCreateOffer<'static> {
    fn get_amount_error(&self) -> Result<(), NFTokenCreateOfferException> {
        match !self.has_flag(Flag::NFTokenCreateOffer(
            NFTokenCreateOfferFlag::TfSellOffer,
        )) && self.amount.get_value_as_u32() == 0
        {
            true => Err(NFTokenCreateOfferException::InvalidAmountMustBeGreaterZero),
            false => Ok(()),
        }
    }

    fn get_destination_error(&self) -> Result<(), NFTokenCreateOfferException> {
        match self.destination {
            Some(destination) => match destination == self.account {
                true => Err(NFTokenCreateOfferException::InvalidDestinationMustNotEqualAccount),
                false => Ok(()),
            },
            None => Ok(()),
        }
    }

    fn get_owner_error(&self) -> Result<(), NFTokenCreateOfferException> {
        match self.owner {
            Some(owner) => match self.has_flag(Flag::NFTokenCreateOffer(
                NFTokenCreateOfferFlag::TfSellOffer,
            )) {
                true => Err(NFTokenCreateOfferException::InvalidOwnerMustNotBeSetForSellOffer),
                false => match owner == self.account {
                    true => Err(NFTokenCreateOfferException::InvalidOwnerMustNotEqualAccount),
                    false => Ok(()),
                },
            },
            None => match !self.has_flag(Flag::NFTokenCreateOffer(
                NFTokenCreateOfferFlag::TfSellOffer,
            )) {
                true => Err(NFTokenCreateOfferException::InvalidOwnerMustBeSetForBuyOffer),
                false => Ok(()),
            },
        }
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

impl Model for NFTokenMint<'static> {
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

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_issuer_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::NFTokenMintError(error),
            )),
            Ok(_no_error) => match self.get_transfer_fee_error() {
                Err(error) => Err(XRPLModelException::XRPLTransactionError(
                    XRPLTransactionException::NFTokenMintError(error),
                )),
                Ok(_no_error) => match self.get_uri_error() {
                    Err(error) => Err(XRPLModelException::XRPLTransactionError(
                        XRPLTransactionException::NFTokenMintError(error),
                    )),
                    Ok(_no_error) => Ok(()),
                },
            },
        }
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

    fn has_flag(&self, flag: Flag) -> bool {
        let mut has_flag = false;
        if self.iter_to_int() > 0 {
            match flag {
                Flag::NFTokenMint(nftoken_mint_flag) => match nftoken_mint_flag {
                    NFTokenMintFlag::TfBurnable => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&NFTokenMintFlag::TfBurnable)
                        {
                            has_flag = true
                        };
                    }
                    NFTokenMintFlag::TfOnlyXRP => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&NFTokenMintFlag::TfOnlyXRP)
                        {
                            has_flag = true
                        };
                    }
                    NFTokenMintFlag::TfTransferable => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&NFTokenMintFlag::TfTransferable)
                        {
                            has_flag = true
                        };
                    }
                    NFTokenMintFlag::TfTrustline => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&NFTokenMintFlag::TfTrustline)
                        {
                            has_flag = true
                        };
                    }
                },
                _ => has_flag = false,
            };
        }
        has_flag
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl NFTokenMintError for NFTokenMint<'static> {
    fn get_issuer_error(&self) -> Result<(), NFTokenMintException> {
        match self.issuer {
            Some(issuer) => match issuer == self.account {
                true => Err(NFTokenMintException::InvalidIssuerMustNotEqualAccount),
                false => Ok(()),
            },
            None => Ok(()),
        }
    }

    fn get_transfer_fee_error(&self) -> Result<(), NFTokenMintException> {
        match self.transfer_fee {
            Some(transfer_fee) => match transfer_fee > MAX_TRANSFER_FEE {
                true => Err(NFTokenMintException::InvalidTransferFeeTooHigh {
                    max: MAX_TRANSFER_FEE,
                    found: transfer_fee,
                }),
                false => Ok(()),
            },
            None => Ok(()),
        }
    }

    fn get_uri_error(&self) -> Result<(), NFTokenMintException> {
        match self.uri {
            Some(uri) => match uri.len() > MAX_URI_LENGTH {
                true => Err(NFTokenMintException::InvalidURITooLong {
                    max: MAX_URI_LENGTH,
                    found: uri.len(),
                }),
                false => Ok(()),
            },
            None => Ok(()),
        }
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

impl Model for OfferCancel<'static> {
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
}

impl Transaction for OfferCancel<'static> {
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

impl Model for OfferCreate<'static> {
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

    fn has_flag(&self, flag: Flag) -> bool {
        let mut has_flag = false;
        if self.iter_to_int() > 0 {
            match flag {
                Flag::OfferCreate(offer_create_flag) => match offer_create_flag {
                    OfferCreateFlag::TfFillOrKill => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&OfferCreateFlag::TfFillOrKill)
                        {
                            has_flag = true
                        };
                    }
                    OfferCreateFlag::TfImmediateOrCancel => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&OfferCreateFlag::TfImmediateOrCancel)
                        {
                            has_flag = true
                        };
                    }
                    OfferCreateFlag::TfPassive => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&OfferCreateFlag::TfPassive)
                        {
                            has_flag = true
                        };
                    }
                    OfferCreateFlag::TfSell => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&OfferCreateFlag::TfSell)
                        {
                            has_flag = true
                        };
                    }
                },
                _ => has_flag = false,
            };
        }
        has_flag
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

impl Model for Payment<'static> {
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

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_xrp_transaction_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::PaymentError(error),
            )),
            Ok(_no_error) => match self.get_partial_payment_error() {
                Err(error) => Err(XRPLModelException::XRPLTransactionError(
                    XRPLTransactionException::PaymentError(error),
                )),
                Ok(_no_error) => match self.get_exchange_error() {
                    Err(error) => Err(XRPLModelException::XRPLTransactionError(
                        XRPLTransactionException::PaymentError(error),
                    )),
                    Ok(_no_error) => Ok(()),
                },
            },
        }
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

    fn has_flag(&self, flag: Flag) -> bool {
        let mut has_flag = false;
        if self.iter_to_int() > 0 {
            match flag {
                Flag::Payment(payment_flag) => match payment_flag {
                    PaymentFlag::TfLimitQuality => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&PaymentFlag::TfLimitQuality)
                        {
                            has_flag = true
                        };
                    }
                    PaymentFlag::TfNoDirectRipple => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&PaymentFlag::TfNoDirectRipple)
                        {
                            has_flag = true
                        };
                    }
                    PaymentFlag::TfPartialPayment => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&PaymentFlag::TfPartialPayment)
                        {
                            has_flag = true
                        };
                    }
                },
                _ => has_flag = false,
            };
        }
        has_flag
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl PaymentError for Payment<'static> {
    fn get_xrp_transaction_error(&self) -> Result<(), PaymentException> {
        match self.send_max.as_ref() {
            Some(_send_max) => Ok(()),
            None => match self.amount.is_xrp() {
                true => match self.paths.as_ref() {
                    Some(_paths) => Err(PaymentException::InvalidXRPtoXRPPaymentsCannotContainPaths),
                    None => match self.account == self.destination {
                        true => Err(PaymentException::InvalidDestinationMustNotEqualAccountForXRPtoXRPPayments),
                        false => Ok(()),
                    }
                },
                false => Ok(()),
            }
        }
    }

    fn get_partial_payment_error(&self) -> Result<(), PaymentException> {
        match self.send_max.as_ref() {
            Some(send_max) => match !self.has_flag(Flag::Payment(PaymentFlag::TfPartialPayment)) {
                true => match send_max.is_xrp() && self.amount.is_xrp() {
                    true => Err(
                        PaymentException::InvalidSendMaxMustNotBeSetForXRPtoXRPNonPartialPayments,
                    ),
                    false => Ok(()),
                },
                false => Ok(()),
            },
            None => match self.has_flag(Flag::Payment(PaymentFlag::TfPartialPayment)) {
                true => Err(PaymentException::InvalidSendMaxMustBeSetForPartialPayments),
                false => match self.deliver_min.as_ref() {
                    Some(_deliver_min) => {
                        Err(PaymentException::InvalidDeliverMinMustNotBeSetForNonPartialPayments)
                    }
                    None => Ok(()),
                },
            },
        }
    }

    fn get_exchange_error(&self) -> Result<(), PaymentException> {
        match self.send_max.as_ref() {
            Some(_send_max) => Ok(()),
            None => match self.account == self.destination {
                true => Err(PaymentException::InvalidSendMaxMustBeSetForExchanges),
                false => Ok(()),
            },
        }
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

impl Model for PaymentChannelClaim<'static> {
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

    fn has_flag(&self, flag: Flag) -> bool {
        let mut has_flag = false;
        if self.iter_to_int() > 0 {
            match flag {
                Flag::PaymentChannelClaim(payment_channel_claim_flag) => {
                    match payment_channel_claim_flag {
                        PaymentChannelClaimFlag::TfClose => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&PaymentChannelClaimFlag::TfClose)
                            {
                                has_flag = true
                            };
                        }
                        PaymentChannelClaimFlag::TfRenew => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&PaymentChannelClaimFlag::TfRenew)
                            {
                                has_flag = true
                            };
                        }
                    }
                }
                _ => has_flag = false,
            };
        }
        has_flag
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

impl Model for PaymentChannelCreate<'static> {
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
}

impl Transaction for PaymentChannelCreate<'static> {
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

impl Model for PaymentChannelFund<'static> {
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
}

impl Transaction for PaymentChannelFund<'static> {
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

impl Model for SetRegularKey<'static> {
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
}

impl Transaction for SetRegularKey<'static> {
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
    signer_entries: Option<Vec<SignerEntry<'a>>>,
}

impl Model for SignerListSet<'static> {
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

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_signer_entries_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::SignerListSetError(error),
            )),
            Ok(_no_error) => match self.get_signer_quorum_error() {
                Err(error) => Err(XRPLModelException::XRPLTransactionError(
                    XRPLTransactionException::SignerListSetError(error),
                )),
                Ok(_no_error) => Ok(()),
            },
        }
    }
}

impl Transaction for SignerListSet<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl SignerListSetError for SignerListSet<'static> {
    fn get_signer_entries_error(&self) -> Result<(), SignerListSetException> {
        match self.signer_entries.as_ref() {
            Some(signer_entries) => match self.signer_quorum == 0 {
                true => Err(SignerListSetException::InvalidMustNotSetSignerEntriesIfSignerListIsBeingDeleted),
                false => match self.signer_quorum == 0 {
                    true => Err(SignerListSetException::InvalidSignerQuorumMustBeGreaterZero),
                    false => match signer_entries.is_empty() {
                        true => Err(SignerListSetException::InvalidTooFewSignerEntries { min: 1, found: signer_entries.len() }),
                        false => match signer_entries.len() > 8 {
                            true => Err(SignerListSetException::InvalidTooManySignerEntries { max: 8, found: signer_entries.len() }),
                            false => Ok(())
                        },
                    },
                },
            },
            None => Ok(())
        }
    }

    fn get_signer_quorum_error(&self) -> Result<(), SignerListSetException> {
        let mut accounts = Vec::new();
        let mut signer_weight_sum: u32 = 0;
        if self.signer_entries.is_some() {
            for signer_entry in self.signer_entries.as_ref().unwrap() {
                accounts.push(signer_entry.account);
                let weight: u32 = signer_entry.signer_weight.into();
                signer_weight_sum += weight;
            }
        }
        accounts.sort_unstable();
        accounts.dedup();
        match self.signer_entries.as_ref() {
            Some(_signer_entries) => match accounts.contains(&self.account) {
                true => Err(SignerListSetException::InvalidAccountMustNotBeInSignerEntry),
                false => match self.signer_quorum > signer_weight_sum {
                    true => Err(SignerListSetException::InvalidMustBeLessOrEqualToSumOfSignerWeightInSignerEntries { max: signer_weight_sum, found: self.signer_quorum }),
                    false => Ok(())
                },
            },
            None => match self.signer_quorum != 0 {
                true => Err(SignerListSetException::InvalidSignerQuorumMustBeZeroIfSignerListIsBeingDeleted),
                false => Ok(()),
            }
        }
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

impl Model for TicketCreate<'static> {
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
}

impl Transaction for TicketCreate<'static> {
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

impl Model for TrustSet<'static> {
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

    fn has_flag(&self, flag: Flag) -> bool {
        let mut has_flag = false;
        if self.iter_to_int() > 0 {
            match flag {
                Flag::TrustSet(trust_set_flag) => match trust_set_flag {
                    TrustSetFlag::TfClearFreeze => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&TrustSetFlag::TfClearFreeze)
                        {
                            has_flag = true
                        };
                    }
                    TrustSetFlag::TfClearNoRipple => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&TrustSetFlag::TfClearNoRipple)
                        {
                            has_flag = true
                        };
                    }
                    TrustSetFlag::TfSetAuth => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&TrustSetFlag::TfSetAuth)
                        {
                            has_flag = true
                        };
                    }
                    TrustSetFlag::TfSetFreeze => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&TrustSetFlag::TfSetFreeze)
                        {
                            has_flag = true
                        };
                    }
                    TrustSetFlag::TfSetNoRipple => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&TrustSetFlag::TfSetNoRipple)
                        {
                            has_flag = true
                        };
                    }
                },
                _ => has_flag = false,
            };
        }
        has_flag
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

impl Model for EnableAmendment<'static> {
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

    fn has_flag(&self, flag: Flag) -> bool {
        let mut has_flag = false;
        if self.iter_to_int() > 0 {
            match flag {
                Flag::EnableAmendment(enable_amendment_flag) => match enable_amendment_flag {
                    EnableAmendmentFlag::TfGotMajority => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&EnableAmendmentFlag::TfGotMajority)
                        {
                            has_flag = true
                        };
                    }
                    EnableAmendmentFlag::TfLostMajority => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&EnableAmendmentFlag::TfLostMajority)
                        {
                            has_flag = true
                        };
                    }
                },
                _ => has_flag = false,
            };
        }
        has_flag
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

impl Model for SetFee<'static> {
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
}

impl Transaction for SetFee<'static> {
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

impl Model for UNLModify<'static> {
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

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_unl_modify_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::UNLModifyError(error),
            )),
            Ok(_no_error) => Ok(()),
        }
    }
}

impl Transaction for UNLModify<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl UNLModifyError for UNLModify<'static> {
    fn get_unl_modify_error(&self) -> Result<(), UNLModifyException> {
        let possible_unlmodify_disabling: [u8; 2] = [0, 1];
        match !possible_unlmodify_disabling.contains(&self.unlmodify_disabling) {
            true => Err(UNLModifyException::InvalidUNLModifyDisablingMustBeOneOrTwo),
            false => Ok(()),
        }
    }
}

#[cfg(test)]
mod test {
    use alloc::vec;

    use super::*;

    #[test]
    fn test_to_json_error() {
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
                value: Some(Borrowed(xrp_amount)),
                currency: Borrowed("XRP"),
            },
            taker_pays: Currency::IssuedCurrency {
                value: Some(Borrowed(usd_amount)),
                currency: Borrowed("USD"),
                issuer: Borrowed("rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq"),
            },
            expiration: None,
            offer_sequence: None,
        };
        let actual = offer_create.to_json_value();
        let json = r#"{"Account":"rpXhhWmCvDwkzNtRbm7mmD1vZqdfatQNEe","Fee":"10","Sequence":1,"LastLedgerSequence":72779837,"Flags":131072,"TakerGets":{"value":"1000000","currency":"XRP"},"TakerPays":{"value":"0.3","currency":"USD","issuer":"rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq"}}"#;
        let expect: Value = serde_json::from_str(json).unwrap();
        assert_eq!(actual, expect)
    }

    #[test]
    fn test_has_flag_error() {
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
                value: Some(Borrowed(xrp_amount)),
                currency: Borrowed("XRP"),
            },
            taker_pays: Currency::IssuedCurrency {
                value: Some(Borrowed(usd_amount)),
                currency: Borrowed("USD"),
                issuer: Borrowed("rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq"),
            },
            expiration: None,
            offer_sequence: None,
        };
        assert!(offer_create.has_flag(Flag::OfferCreate(OfferCreateFlag::TfImmediateOrCancel)))
    }

    #[test]
    fn test_get_transaction_type_error() {
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
                value: Some(Borrowed(xrp_amount)),
                currency: Borrowed("XRP"),
            },
            taker_pays: Currency::IssuedCurrency {
                value: Some(Borrowed(usd_amount)),
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

#[cfg(test)]
mod test_account_set_errors {
    use crate::models::{
        exceptions::{AccountSetException, XRPLModelException, XRPLTransactionException},
        AccountSetFlag, Model,
    };

    use super::AccountSet;

    #[test]
    fn test_tick_size_error() {
        let mut account_set = AccountSet {
            transaction_type: crate::models::TransactionType::AccountSet,
            account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            fee: None,
            sequence: None,
            last_ledger_sequence: None,
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: None,
            memos: None,
            signers: None,
            clear_flag: None,
            domain: None,
            email_hash: None,
            message_key: None,
            set_flag: None,
            transfer_rate: None,
            tick_size: None,
            nftoken_minter: None,
        };
        let tick_size_too_low = Some(2);
        account_set.tick_size = tick_size_too_low;
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::AccountSetError(
                AccountSetException::InvalidTickSizeTooLow { min: 3, found: 2 },
            ));
        match account_set.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };

        let tick_size_too_high = Some(16);
        account_set.tick_size = tick_size_too_high;
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::AccountSetError(
                AccountSetException::InvalidTickSizeTooHigh { max: 15, found: 16 },
            ));
        match account_set.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };
    }

    #[test]
    fn test_transfer_rate_error() {
        let mut account_set = AccountSet {
            transaction_type: crate::models::TransactionType::AccountSet,
            account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            fee: None,
            sequence: None,
            last_ledger_sequence: None,
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: None,
            memos: None,
            signers: None,
            clear_flag: None,
            domain: None,
            email_hash: None,
            message_key: None,
            set_flag: None,
            transfer_rate: None,
            tick_size: None,
            nftoken_minter: None,
        };
        let tick_size_too_low = Some(999999999);
        account_set.transfer_rate = tick_size_too_low;
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::AccountSetError(
                AccountSetException::InvalidTransferRateTooLow {
                    min: 1000000000,
                    found: 999999999,
                },
            ));
        match account_set.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };

        let tick_size_too_high = Some(2000000001);
        account_set.transfer_rate = tick_size_too_high;
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::AccountSetError(
                AccountSetException::InvalidTransferRateTooHigh {
                    max: 2000000000,
                    found: 2000000001,
                },
            ));
        match account_set.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };
    }

    #[test]
    fn test_domain_error() {
        let mut account_set = AccountSet {
            transaction_type: crate::models::TransactionType::AccountSet,
            account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            fee: None,
            sequence: None,
            last_ledger_sequence: None,
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: None,
            memos: None,
            signers: None,
            clear_flag: None,
            domain: None,
            email_hash: None,
            message_key: None,
            set_flag: None,
            transfer_rate: None,
            tick_size: None,
            nftoken_minter: None,
        };
        let domain_not_lowercase = Some("https://Example.com/");
        account_set.domain = domain_not_lowercase;
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::AccountSetError(
                AccountSetException::InvalidDomainIsNotLowercase,
            ));
        match account_set.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };

        let domain_too_long = Some("https://example.com/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        account_set.domain = domain_too_long;
        let expected_error = XRPLModelException::XRPLTransactionError(
            XRPLTransactionException::AccountSetError(AccountSetException::InvalidDomainTooLong {
                max: 256,
                found: 270,
            }),
        );
        match account_set.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };
    }

    #[test]
    fn test_flag_error() {
        let account_set = AccountSet {
            transaction_type: crate::models::TransactionType::AccountSet,
            account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            fee: None,
            sequence: None,
            last_ledger_sequence: None,
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: None,
            memos: None,
            signers: None,
            clear_flag: Some(AccountSetFlag::AsfDisallowXRP),
            domain: None,
            email_hash: None,
            message_key: None,
            set_flag: Some(AccountSetFlag::AsfDisallowXRP),
            transfer_rate: None,
            tick_size: None,
            nftoken_minter: None,
        };
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::AccountSetError(
                AccountSetException::InvalidClearFlagMustNotEqualSetFlag,
            ));
        match account_set.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };
    }

    #[test]
    fn test_asf_authorized_nftoken_minter_error() {
        let mut account_set = AccountSet {
            transaction_type: crate::models::TransactionType::AccountSet,
            account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            fee: None,
            sequence: None,
            last_ledger_sequence: None,
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: None,
            memos: None,
            signers: None,
            clear_flag: None,
            domain: None,
            email_hash: None,
            message_key: None,
            set_flag: None,
            transfer_rate: None,
            tick_size: None,
            nftoken_minter: None,
        };
        account_set.nftoken_minter = Some("rLSn6Z3T8uCxbcd1oxwfGQN1Fdn5CyGujK");
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::AccountSetError(
                AccountSetException::InvalidMustSetAsfAuthorizedNftokenMinterFlagToSetMinter,
            ));
        match account_set.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };
        account_set.nftoken_minter = None;

        account_set.set_flag = Some(AccountSetFlag::AsfAuthorizedNFTokenMinter);
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::AccountSetError(
                AccountSetException::InvalidNftokenMinterMustBeSetIfAsfAuthorizedNftokenMinterIsSet,
            ));
        match account_set.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };
        account_set.set_flag = None;

        account_set.nftoken_minter = Some("rLSn6Z3T8uCxbcd1oxwfGQN1Fdn5CyGujK");
        account_set.clear_flag = Some(AccountSetFlag::AsfAuthorizedNFTokenMinter);
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::AccountSetError(
                AccountSetException::InvalidNftokenMinterMustNotBeSetIfAsfAuthorizedNftokenMinterIsUnset,
            ));
        match account_set.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };
    }
}

#[cfg(test)]
mod test_check_cash_error {
    use crate::models::{
        exceptions::{CheckCashException, XRPLModelException, XRPLTransactionException},
        Currency, Model, TransactionType,
    };

    use alloc::borrow::Cow;

    use super::CheckCash;

    #[test]
    fn test_amount_and_deliver_min_error() {
        let mut check_cash = CheckCash {
            transaction_type: TransactionType::CheckCash,
            account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            fee: None,
            sequence: None,
            last_ledger_sequence: None,
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: None,
            memos: None,
            signers: None,
            check_id: "",
            amount: None,
            deliver_min: None,
        };
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::CheckCashError(
                CheckCashException::InvalidMustSetAmountOrDeliverMin,
            ));
        match check_cash.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };

        check_cash.amount = Some(Currency::Xrp {
            value: Some(Cow::Borrowed("1000000")),
            currency: Cow::Borrowed("XRP"),
        });
        check_cash.deliver_min = Some(Currency::Xrp {
            value: Some(Cow::Borrowed("100000")),
            currency: Cow::Borrowed("XRP"),
        });
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::CheckCashError(
                CheckCashException::InvalidMustNotSetAmountAndDeliverMin,
            ));
        match check_cash.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };
    }
}

#[cfg(test)]
mod test_deposit_preauth_exception {
    use crate::models::{
        exceptions::{DepositPreauthException, XRPLModelException, XRPLTransactionException},
        Model, TransactionType,
    };

    use super::DepositPreauth;

    #[test]
    fn test_authorize_and_unauthorize_error() {
        let mut deposit_preauth = DepositPreauth {
            transaction_type: TransactionType::DepositPreauth,
            account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            fee: None,
            sequence: None,
            last_ledger_sequence: None,
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: None,
            memos: None,
            signers: None,
            authorize: None,
            unauthorize: None,
        };
        let expected_error = XRPLModelException::XRPLTransactionError(
            XRPLTransactionException::DepositPreauthError(
                DepositPreauthException::InvalidMustSetAuthorizeOrUnauthorize,
            ),
        );
        match deposit_preauth.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };

        deposit_preauth.authorize = Some("rLSn6Z3T8uCxbcd1oxwfGQN1Fdn5CyGujK");
        deposit_preauth.unauthorize = Some("raQwCVAJVqjrVm1Nj5SFRcX8i22BhdC9WA");
        let expected_error = XRPLModelException::XRPLTransactionError(
            XRPLTransactionException::DepositPreauthError(
                DepositPreauthException::InvalidMustNotSetAuthorizeAndUnauthorize,
            ),
        );
        match deposit_preauth.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };
    }
}

#[cfg(test)]
mod test_escrow_create_errors {
    use crate::models::{
        exceptions::{EscrowCreateException, XRPLModelException, XRPLTransactionException},
        Currency, Model, TransactionType,
    };

    use alloc::borrow::Cow;

    use super::EscrowCreate;

    #[test]
    fn test_cancel_after_error() {
        let escrow_create = EscrowCreate {
            transaction_type: TransactionType::EscrowCreate,
            account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            fee: None,
            sequence: None,
            last_ledger_sequence: None,
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: None,
            memos: None,
            signers: None,
            amount: Currency::Xrp {
                value: Some(Cow::Borrowed("100000000")),
                currency: Cow::Borrowed("XRP"),
            },
            destination: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            destination_tag: None,
            cancel_after: Some(13298498),
            finish_after: Some(14359039),
            condition: None,
        };
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::EscrowCreateError(
                EscrowCreateException::InvalidCancelAfterMustNotBeBeforeFinishAfter,
            ));
        match escrow_create.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };
    }
}

#[cfg(test)]
mod test_escrow_finish_errors {
    use crate::models::{
        exceptions::{EscrowFinishException, XRPLModelException, XRPLTransactionException},
        Model, TransactionType,
    };

    use super::EscrowFinish;

    #[test]
    fn test_condition_and_fulfillment_error() {
        let escrow_finish = EscrowFinish {
            transaction_type: TransactionType::EscrowCancel,
            account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            fee: None,
            sequence: None,
            last_ledger_sequence: None,
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: None,
            memos: None,
            signers: None,
            owner: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            offer_sequence: 10,
            condition: Some(
                "A0258020E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855810100",
            ),
            fulfillment: None,
        };
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::EscrowFinishError(
                EscrowFinishException::InvalidIfOneSetBothConditionAndFulfillmentMustBeSet,
            ));
        match escrow_finish.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };
    }
}

#[cfg(test)]
mod test_nftoken_accept_offer_error {
    use alloc::borrow::Cow;

    use crate::models::{
        exceptions::{NFTokenAcceptOfferException, XRPLModelException, XRPLTransactionException},
        Currency, Model, TransactionType,
    };

    use super::NFTokenAcceptOffer;

    #[test]
    fn test_brokered_mode_error() {
        let nftoken_accept_offer = NFTokenAcceptOffer {
            transaction_type: TransactionType::NFTokenAcceptOffer,
            account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            fee: None,
            sequence: None,
            last_ledger_sequence: None,
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: None,
            memos: None,
            signers: None,
            nftoken_sell_offer: None,
            nftoken_buy_offer: None,
            nftoken_broker_fee: Some(Currency::Xrp {
                value: Some(Cow::Borrowed("100")),
                currency: Cow::Borrowed("XRP"),
            }),
        };
        let expected_error = XRPLModelException::XRPLTransactionError(
            XRPLTransactionException::NFTokenAcceptOfferError(
                NFTokenAcceptOfferException::InvalidMustSetEitherNftokenBuyOfferOrNftokenSellOffer,
            ),
        );
        match nftoken_accept_offer.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };
    }

    #[test]
    fn test_broker_fee_error() {
        let nftoken_accept_offer = NFTokenAcceptOffer {
            transaction_type: TransactionType::NFTokenAcceptOffer,
            account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            fee: None,
            sequence: None,
            last_ledger_sequence: None,
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: None,
            memos: None,
            signers: None,
            nftoken_sell_offer: Some(""),
            nftoken_buy_offer: None,
            nftoken_broker_fee: Some(Currency::Xrp {
                value: Some(Cow::Borrowed("0")),
                currency: Cow::Borrowed("XRP"),
            }),
        };
        let expected_error = XRPLModelException::XRPLTransactionError(
            XRPLTransactionException::NFTokenAcceptOfferError(
                NFTokenAcceptOfferException::InvalidBrokerFeeMustBeGreaterZero,
            ),
        );
        match nftoken_accept_offer.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };
    }
}

#[cfg(test)]
mod test_nftoken_cancel_offer_error {
    use alloc::vec::Vec;

    use crate::models::{
        exceptions::{NFTokenCancelOfferException, XRPLModelException, XRPLTransactionException},
        Model, TransactionType,
    };

    use super::NFTokenCancelOffer;

    #[test]
    fn test_nftoken_offer_error() {
        let nftoken_cancel_offer = NFTokenCancelOffer {
            transaction_type: TransactionType::NFTokenCancelOffer,
            account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            fee: None,
            sequence: None,
            last_ledger_sequence: None,
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: None,
            memos: None,
            signers: None,
            nftoken_offers: Vec::new(),
        };
        let expected_error = XRPLModelException::XRPLTransactionError(
            XRPLTransactionException::NFTokenCancelOfferError(
                NFTokenCancelOfferException::InvalidMustIncludeOneNFTokenOffer,
            ),
        );
        match nftoken_cancel_offer.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };
    }
}

#[cfg(test)]
mod test_nftoken_create_offer_error {
    use alloc::{borrow::Cow, vec};

    use crate::models::{
        exceptions::{NFTokenCreateOfferException, XRPLModelException, XRPLTransactionException},
        Currency, Model, NFTokenCreateOfferFlag, TransactionType,
    };

    use super::NFTokenCreateOffer;

    #[test]
    fn test_amount_error() {
        let nftoken_create_offer = NFTokenCreateOffer {
            transaction_type: TransactionType::NFTokenCreateOffer,
            account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            fee: None,
            sequence: None,
            last_ledger_sequence: None,
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: None,
            memos: None,
            signers: None,
            nftoken_id: "",
            amount: Currency::Xrp {
                value: Some(Cow::Borrowed("0")),
                currency: Cow::Borrowed("XRP"),
            },
            owner: None,
            expiration: None,
            destination: None,
        };
        let expected_error = XRPLModelException::XRPLTransactionError(
            XRPLTransactionException::NFTokenCreateOfferError(
                NFTokenCreateOfferException::InvalidAmountMustBeGreaterZero,
            ),
        );
        match nftoken_create_offer.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };
    }

    #[test]
    fn test_destination_error() {
        let nftoken_create_offer = NFTokenCreateOffer {
            transaction_type: TransactionType::NFTokenCreateOffer,
            account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            fee: None,
            sequence: None,
            last_ledger_sequence: None,
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: None,
            memos: None,
            signers: None,
            nftoken_id: "",
            amount: Currency::Xrp {
                value: Some(Cow::Borrowed("1")),
                currency: Cow::Borrowed("XRP"),
            },
            owner: None,
            expiration: None,
            destination: Some("rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb"),
        };
        let expected_error = XRPLModelException::XRPLTransactionError(
            XRPLTransactionException::NFTokenCreateOfferError(
                NFTokenCreateOfferException::InvalidDestinationMustNotEqualAccount,
            ),
        );
        match nftoken_create_offer.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };
    }

    #[test]
    fn test_owner_error() {
        let mut nftoken_create_offer = NFTokenCreateOffer {
            transaction_type: TransactionType::NFTokenCreateOffer,
            account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            fee: None,
            sequence: None,
            last_ledger_sequence: None,
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: None,
            memos: None,
            signers: None,
            nftoken_id: "",
            amount: Currency::Xrp {
                value: Some(Cow::Borrowed("1")),
                currency: Cow::Borrowed("XRP"),
            },
            owner: Some("rLSn6Z3T8uCxbcd1oxwfGQN1Fdn5CyGujK"),
            expiration: None,
            destination: None,
        };
        let sell_flag = vec![NFTokenCreateOfferFlag::TfSellOffer];
        nftoken_create_offer.flags = Some(sell_flag);
        let expected_error = XRPLModelException::XRPLTransactionError(
            XRPLTransactionException::NFTokenCreateOfferError(
                NFTokenCreateOfferException::InvalidOwnerMustNotBeSetForSellOffer,
            ),
        );
        match nftoken_create_offer.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };

        nftoken_create_offer.flags = None;
        nftoken_create_offer.owner = None;
        let expected_error = XRPLModelException::XRPLTransactionError(
            XRPLTransactionException::NFTokenCreateOfferError(
                NFTokenCreateOfferException::InvalidOwnerMustBeSetForBuyOffer,
            ),
        );
        match nftoken_create_offer.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };

        nftoken_create_offer.owner = Some("rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb");
        let expected_error = XRPLModelException::XRPLTransactionError(
            XRPLTransactionException::NFTokenCreateOfferError(
                NFTokenCreateOfferException::InvalidOwnerMustNotEqualAccount,
            ),
        );
        match nftoken_create_offer.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };
    }
}

#[cfg(test)]
mod test_nftoken_mint_error {
    use crate::models::{
        exceptions::{NFTokenMintException, XRPLModelException, XRPLTransactionException},
        Model, TransactionType,
    };

    use super::NFTokenMint;

    #[test]
    fn test_issuer_error() {
        let nftoken_mint = NFTokenMint {
            transaction_type: TransactionType::NFTokenMint,
            account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            fee: None,
            sequence: None,
            last_ledger_sequence: None,
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: None,
            memos: None,
            signers: None,
            nftoken_taxon: 0,
            issuer: Some("rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb"),
            transfer_fee: None,
            uri: None,
        };
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::NFTokenMintError(
                NFTokenMintException::InvalidIssuerMustNotEqualAccount,
            ));
        match nftoken_mint.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };
    }

    #[test]
    fn test_transfer_fee_error() {
        let nftoken_mint = NFTokenMint {
            transaction_type: TransactionType::NFTokenMint,
            account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            fee: None,
            sequence: None,
            last_ledger_sequence: None,
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: None,
            memos: None,
            signers: None,
            nftoken_taxon: 0,
            issuer: None,
            transfer_fee: Some(50001),
            uri: None,
        };
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::NFTokenMintError(
                NFTokenMintException::InvalidTransferFeeTooHigh {
                    max: 50000,
                    found: 50001,
                },
            ));
        match nftoken_mint.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };
    }

    #[test]
    fn test_uri_error() {
        let nftoken_mint = NFTokenMint {
            transaction_type: TransactionType::NFTokenMint,
            account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            fee: None,
            sequence: None,
            last_ledger_sequence: None,
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: None,
            memos: None,
            signers: None,
            nftoken_taxon: 0,
            issuer: None,
            transfer_fee: None,
            uri: Some("wss://xrplcluster.com/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        };
        let expected_error = XRPLModelException::XRPLTransactionError(
            XRPLTransactionException::NFTokenMintError(NFTokenMintException::InvalidURITooLong {
                max: 512,
                found: 513,
            }),
        );
        match nftoken_mint.validate() {
            Ok(_no_error) => (),
            Err(error) => assert_eq!(error, expected_error),
        };
    }
}
