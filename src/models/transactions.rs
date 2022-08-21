//! Transaction models.

use crate::{
    constants::{
        DISABLE_TICK_SIZE, MAX_DOMAIN_LENGTH, MAX_TICK_SIZE, MAX_TRANSFER_FEE, MAX_TRANSFER_RATE,
        MAX_URI_LENGTH, MIN_TICK_SIZE, MIN_TRANSFER_RATE, SPECIAL_CASE_TRANFER_RATE,
    },
    models::*,
};
use alloc::{string::ToString, vec::Vec};
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::account_set")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<u32>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    signers: Option<Vec<Signer<'a>>>,
    // The custom fields for the AccountDelete model.
    //
    // See AccountDelete fields:
    // `<https://xrpl.org/accountdelete.html#accountdelete-fields>`
    /// The address of an account to receive any leftover XRP after
    /// deleting the sending account. Must be a funded account in
    /// the ledger, and must not be the sending account.
    destination: &'a str,
    /// Arbitrary destination tag that identifies a hosted
    /// recipient or other information for the recipient
    /// of the deleted account's leftover XRP.
    destination_tag: Option<u32>,
}

impl Model for AccountDelete<'static> {
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `AccountDelete` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        transaction_json
    }
}

impl From<&AccountDelete<'static>> for u32 {
    fn from(_: &AccountDelete<'static>) -> Self {
        0
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::account_set")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<AccountSetFlag>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    signers: Option<Vec<Signer<'a>>>,
    // The custom fields for the AccountSet model.
    //
    // See AccountSet fields:
    // `<https://xrpl.org/accountset.html#accountset-fields>`
    /// Unique identifier of a flag to disable for this account.
    clear_flag: Option<AccountSetFlag>,
    /// The domain that owns this account, as a string of hex
    /// representing the ASCII for the domain in lowercase.
    /// Cannot be more than 256 bytes in length.
    domain: Option<&'a str>,
    /// Hash of an email address to be used for generating an
    /// avatar image. Conventionally, clients use Gravatar
    /// to display this image.
    email_hash: Option<&'a str>,
    /// Public key for sending encrypted messages to this account.
    /// To set the key, it must be exactly 33 bytes, with the
    /// first byte indicating the key type: 0x02 or 0x03 for
    /// secp256k1 keys, 0xED for Ed25519 keys. To remove the
    /// key, use an empty value.
    message_key: Option<&'a str>,
    /// Sets an alternate account that is allowed to mint NFTokens
    /// on this account's behalf using NFTokenMint's Issuer field.
    /// This field is part of the experimental XLS-20 standard
    /// for non-fungible tokens.
    nftoken_minter: Option<&'a str>,
    /// Flag to enable for this account.
    set_flag: Option<AccountSetFlag>,
    /// The fee to charge when users transfer this account's tokens,
    /// represented as billionths of a unit. Cannot be more than
    /// 2000000000 or less than 1000000000, except for the special
    /// case 0 meaning no fee.
    transfer_rate: Option<u32>,
    /// Tick size to use for offers involving a currency issued by
    /// this address. The exchange rates of those offers is rounded
    /// to this many significant digits. Valid values are 3 to 15
    /// inclusive, or 0 to disable.
    tick_size: Option<u32>,
}

impl Model for AccountSet<'static> {
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `AccountSet` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self._get_tick_size_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::AccountSetError(error),
            )),
            Ok(_no_error) => match self._get_transfer_rate_error() {
                Err(error) => Err(XRPLModelException::XRPLTransactionError(
                    XRPLTransactionException::AccountSetError(error),
                )),
                Ok(_no_error) => match self._get_domain_error() {
                    Err(error) => Err(XRPLModelException::XRPLTransactionError(
                        XRPLTransactionException::AccountSetError(error),
                    )),
                    Ok(_no_error) => match self._get_clear_flag_error() {
                        Err(error) => Err(XRPLModelException::XRPLTransactionError(
                            XRPLTransactionException::AccountSetError(error),
                        )),
                        Ok(_no_error) => match self._get_nftoken_minter_error() {
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

impl From<&AccountSet<'static>> for u32 {
    fn from(val: &AccountSet<'static>) -> Self {
        if let Some(flag) = val.set_flag.as_ref().or(val.clear_flag.as_ref()) {
            match flag {
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
}

impl Transaction for AccountSet<'static> {
    fn has_flag(&self, flag: &Flag) -> bool {
        let mut flags = &Vec::new();

        if let Some(flag_set) = self.flags.as_ref() {
            flags = flag_set;
        }

        match flag {
            Flag::AccountSet(account_set_flag) => match account_set_flag {
                AccountSetFlag::AsfAccountTxnID => flags.contains(&AccountSetFlag::AsfAccountTxnID),
                AccountSetFlag::AsfAuthorizedNFTokenMinter => {
                    flags.contains(&AccountSetFlag::AsfAuthorizedNFTokenMinter)
                }
                AccountSetFlag::AsfDefaultRipple => {
                    flags.contains(&AccountSetFlag::AsfDefaultRipple)
                }
                AccountSetFlag::AsfDepositAuth => flags.contains(&AccountSetFlag::AsfDepositAuth),
                AccountSetFlag::AsfDisableMaster => {
                    flags.contains(&AccountSetFlag::AsfDisableMaster)
                }
                AccountSetFlag::AsfDisallowXRP => flags.contains(&AccountSetFlag::AsfDisallowXRP),
                AccountSetFlag::AsfGlobalFreeze => flags.contains(&AccountSetFlag::AsfGlobalFreeze),
                AccountSetFlag::AsfNoFreeze => flags.contains(&AccountSetFlag::AsfNoFreeze),
                AccountSetFlag::AsfRequireAuth => flags.contains(&AccountSetFlag::AsfRequireAuth),
                AccountSetFlag::AsfRequireDest => flags.contains(&AccountSetFlag::AsfRequireDest),
            },
            _ => false,
        }
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl AccountSetError for AccountSet<'static> {
    fn _get_tick_size_error(&self) -> Result<(), AccountSetException> {
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

    fn _get_transfer_rate_error(&self) -> Result<(), AccountSetException> {
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

    fn _get_domain_error(&self) -> Result<(), AccountSetException> {
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

    fn _get_clear_flag_error(&self) -> Result<(), AccountSetException> {
        match self.clear_flag.as_ref() {
            Some(_clear_flag) => match self.clear_flag == self.set_flag {
                true => Err(AccountSetException::InvalidClearFlagMustNotEqualSetFlag),
                false => Ok(()),
            },
            None => Ok(()),
        }
    }

    fn _get_nftoken_minter_error(&self) -> Result<(), AccountSetException> {
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::check_cancel")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<u32>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    signers: Option<Vec<Signer<'a>>>,
    // The custom fields for the CheckCancel model.
    //
    // See CheckCancel fields:
    // `<https://xrpl.org/checkcancel.html#checkcancel-fields>`
    check_id: &'a str,
}

impl Model for CheckCancel<'static> {
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `CheckCancel` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        transaction_json
    }
}

impl From<&CheckCancel<'static>> for u32 {
    fn from(_: &CheckCancel<'static>) -> Self {
        0
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::check_cash")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<u32>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
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
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `CheckCash` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        if let Some(amount) = &self.amount {
            if amount.is_xrp() {
                transaction_json["Amount"] =
                    Value::from(amount.get_value_as_u32().to_string().as_str());
            }
        }
        if let Some(deliver_min) = &self.deliver_min {
            if deliver_min.is_xrp() {
                transaction_json["DeliverMin"] =
                    Value::from(deliver_min.get_value_as_u32().to_string().as_str());
            }
        }
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self._get_amount_and_deliver_min_error() {
            Ok(_no_error) => Ok(()),
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::CheckCashError(error),
            )),
        }
    }
}

impl From<&CheckCash<'static>> for u32 {
    fn from(_: &CheckCash<'static>) -> Self {
        0
    }
}

impl Transaction for CheckCash<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl CheckCashError for CheckCash<'static> {
    fn _get_amount_and_deliver_min_error(&self) -> Result<(), CheckCashException> {
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::check_create")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<u32>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
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
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `CheckCreate` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        if self.send_max.is_xrp() {
            transaction_json["SendMax"] =
                Value::from(self.send_max.get_value_as_u32().to_string().as_str());
        }
        transaction_json
    }
}

impl From<&CheckCreate<'static>> for u32 {
    fn from(_: &CheckCreate<'static>) -> Self {
        0
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::deposit_preauth")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<u32>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the DepositPreauth model.
    ///
    /// See DepositPreauth fields:
    /// `<https://xrpl.org/depositpreauth.html#depositpreauth-fields>`
    authorize: Option<&'a str>,
    unauthorize: Option<&'a str>,
}

impl Model for DepositPreauth<'static> {
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `DepositPreauth` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self._get_authorize_and_unauthorize_error() {
            Ok(_no_error) => Ok(()),
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::DepositPreauthError(error),
            )),
        }
    }
}

impl From<&DepositPreauth<'static>> for u32 {
    fn from(_: &DepositPreauth<'static>) -> Self {
        0
    }
}

impl Transaction for DepositPreauth<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl DepositPreauthError for DepositPreauth<'static> {
    fn _get_authorize_and_unauthorize_error(&self) -> Result<(), DepositPreauthException> {
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::escrow_cancel")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<u32>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the EscrowCancel model.
    ///
    /// See EscrowCancel fields:
    /// `<https://xrpl.org/escrowcancel.html#escrowcancel-flags>`
    owner: &'a str,
    offer_sequence: u32,
}

impl Model for EscrowCancel<'static> {
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `EscrowCancel` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        transaction_json
    }
}

impl From<&EscrowCancel<'static>> for u32 {
    fn from(_: &EscrowCancel<'static>) -> Self {
        0
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::escrow_create")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<u32>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
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
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `EscrowCreate` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        if self.amount.is_xrp() {
            transaction_json["Amount"] =
                Value::from(self.amount.get_value_as_u32().to_string().as_str());
        }
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self._get_finish_after_error() {
            Ok(_no_error) => Ok(()),
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::EscrowCreateError(error),
            )),
        }
    }
}

impl From<&EscrowCreate<'static>> for u32 {
    fn from(_: &EscrowCreate<'static>) -> Self {
        0
    }
}

impl Transaction for EscrowCreate<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl EscrowCreateError for EscrowCreate<'static> {
    fn _get_finish_after_error(&self) -> Result<(), EscrowCreateException> {
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::escrow_finish")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<u32>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
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
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `EscrowFinish` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self._get_condition_and_fulfillment_error() {
            Ok(_no_error) => Ok(()),
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::EscrowFinishError(error),
            )),
        }
    }
}

impl From<&EscrowFinish<'static>> for u32 {
    fn from(_: &EscrowFinish<'static>) -> Self {
        0
    }
}

impl Transaction for EscrowFinish<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl EscrowFinishError for EscrowFinish<'static> {
    fn _get_condition_and_fulfillment_error(&self) -> Result<(), EscrowFinishException> {
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::nftoken_accept_offer")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<u32>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
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
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `NFTokenAcceptOffer` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        if let Some(nftoken_broker_fee) = &self.nftoken_broker_fee {
            if nftoken_broker_fee.is_xrp() {
                transaction_json["NFTokenBrokerFee"] =
                    Value::from(nftoken_broker_fee.get_value_as_u32().to_string().as_str());
            }
        }
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self._get_brokered_mode_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::NFTokenAcceptOfferError(error),
            )),
            Ok(_no_error) => match self._get_nftoken_broker_fee_error() {
                Err(error) => Err(XRPLModelException::XRPLTransactionError(
                    XRPLTransactionException::NFTokenAcceptOfferError(error),
                )),
                Ok(_no_error) => Ok(()),
            },
        }
    }
}

impl From<&NFTokenAcceptOffer<'static>> for u32 {
    fn from(_: &NFTokenAcceptOffer<'static>) -> Self {
        0
    }
}

impl Transaction for NFTokenAcceptOffer<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl NFTokenAcceptOfferError for NFTokenAcceptOffer<'static> {
    fn _get_brokered_mode_error(&self) -> Result<(), NFTokenAcceptOfferException> {
        match self.nftoken_broker_fee.as_ref() {
            Some(_nftoken_broker_fee) => match self.nftoken_sell_offer.is_none() && self.nftoken_buy_offer.is_none() {
                true => Err(NFTokenAcceptOfferException::InvalidMustSetEitherNftokenBuyOfferOrNftokenSellOffer),
                false => Ok(()),
            }
            None => Ok(()),
        }
    }
    fn _get_nftoken_broker_fee_error(&self) -> Result<(), NFTokenAcceptOfferException> {
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::nftoken_burn")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<u32>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the NFTokenBurn model.
    ///
    /// See NFTokenBurn fields:
    /// `<https://xrpl.org/nftokenburn.html#nftokenburn-fields>`
    nftoken_id: &'a str,
    owner: Option<&'a str>,
}

impl Model for NFTokenBurn<'static> {
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `NFTokenBurn` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        transaction_json
    }
}

impl From<&NFTokenBurn<'static>> for u32 {
    fn from(_: &NFTokenBurn<'static>) -> Self {
        0
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::nftoken_cancel_offer")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<u32>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
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
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `NFTokenCancelOffer` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self._get_nftoken_offers_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::NFTokenCancelOfferError(error),
            )),
            Ok(_no_error) => Ok(()),
        }
    }
}

impl From<&NFTokenCancelOffer<'static>> for u32 {
    fn from(_: &NFTokenCancelOffer<'static>) -> Self {
        0
    }
}

impl Transaction for NFTokenCancelOffer<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl NFTokenCancelOfferError for NFTokenCancelOffer<'static> {
    fn _get_nftoken_offers_error(&self) -> Result<(), NFTokenCancelOfferException> {
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::nftoken_create_offer")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<NFTokenCreateOfferFlag>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
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
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `NFTokenCreateOffer` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        if self.amount.is_xrp() {
            transaction_json["Amount"] =
                Value::from(self.amount.get_value_as_u32().to_string().as_str());
        }
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self._get_amount_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::NFTokenCreateOfferError(error),
            )),
            Ok(_no_error) => match self._get_destination_error() {
                Err(error) => Err(XRPLModelException::XRPLTransactionError(
                    XRPLTransactionException::NFTokenCreateOfferError(error),
                )),
                Ok(_no_error) => match self._get_owner_error() {
                    Err(error) => Err(XRPLModelException::XRPLTransactionError(
                        XRPLTransactionException::NFTokenCreateOfferError(error),
                    )),
                    Ok(_no_error) => Ok(()),
                },
            },
        }
    }
}

impl From<&NFTokenCreateOffer<'static>> for u32 {
    fn from(val: &NFTokenCreateOffer<'static>) -> Self {
        val.flags
            .as_ref()
            .unwrap_or(&Vec::new())
            .iter()
            .fold(0, |collect, flag| match flag {
                NFTokenCreateOfferFlag::TfSellOffer => collect + 0x00000001,
            })
    }
}

impl Transaction for NFTokenCreateOffer<'static> {
    fn has_flag(&self, flag: &Flag) -> bool {
        let mut flags = &Vec::new();

        if let Some(flag_set) = self.flags.as_ref() {
            flags = flag_set;
        }

        match flag {
            Flag::NFTokenCreateOffer(nftoken_create_offer_flag) => {
                match nftoken_create_offer_flag {
                    NFTokenCreateOfferFlag::TfSellOffer => {
                        flags.contains(&NFTokenCreateOfferFlag::TfSellOffer)
                    }
                }
            }
            _ => false,
        }
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl NFTokenCreateOfferError for NFTokenCreateOffer<'static> {
    fn _get_amount_error(&self) -> Result<(), NFTokenCreateOfferException> {
        match !self.has_flag(&Flag::NFTokenCreateOffer(
            NFTokenCreateOfferFlag::TfSellOffer,
        )) && self.amount.get_value_as_u32() == 0
        {
            true => Err(NFTokenCreateOfferException::InvalidAmountMustBeGreaterZero),
            false => Ok(()),
        }
    }

    fn _get_destination_error(&self) -> Result<(), NFTokenCreateOfferException> {
        match self.destination {
            Some(destination) => match destination == self.account {
                true => Err(NFTokenCreateOfferException::InvalidDestinationMustNotEqualAccount),
                false => Ok(()),
            },
            None => Ok(()),
        }
    }

    fn _get_owner_error(&self) -> Result<(), NFTokenCreateOfferException> {
        match self.owner {
            Some(owner) => match self.has_flag(&Flag::NFTokenCreateOffer(
                NFTokenCreateOfferFlag::TfSellOffer,
            )) {
                true => Err(NFTokenCreateOfferException::InvalidOwnerMustNotBeSetForSellOffer),
                false => match owner == self.account {
                    true => Err(NFTokenCreateOfferException::InvalidOwnerMustNotEqualAccount),
                    false => Ok(()),
                },
            },
            None => match !self.has_flag(&Flag::NFTokenCreateOffer(
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::nftoken_mint")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<NFTokenMintFlag>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
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
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `NFTokenMint` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self._get_issuer_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::NFTokenMintError(error),
            )),
            Ok(_no_error) => match self._get_transfer_fee_error() {
                Err(error) => Err(XRPLModelException::XRPLTransactionError(
                    XRPLTransactionException::NFTokenMintError(error),
                )),
                Ok(_no_error) => match self._get_uri_error() {
                    Err(error) => Err(XRPLModelException::XRPLTransactionError(
                        XRPLTransactionException::NFTokenMintError(error),
                    )),
                    Ok(_no_error) => Ok(()),
                },
            },
        }
    }
}

impl From<&NFTokenMint<'static>> for u32 {
    fn from(val: &NFTokenMint<'static>) -> Self {
        val.flags
            .as_ref()
            .unwrap_or(&Vec::new())
            .iter()
            .fold(0, |collect, flag| match flag {
                NFTokenMintFlag::TfBurnable => collect + 0x00000001,
                NFTokenMintFlag::TfOnlyXRP => collect + 0x00000002,
                NFTokenMintFlag::TfTrustline => collect + 0x00000004,
                NFTokenMintFlag::TfTransferable => collect + 0x00000008,
            })
    }
}

impl Transaction for NFTokenMint<'static> {
    fn has_flag(&self, flag: &Flag) -> bool {
        let mut flags = &Vec::new();

        if let Some(flag_set) = self.flags.as_ref() {
            flags = flag_set;
        }

        match flag {
            Flag::NFTokenMint(nftoken_mint_flag) => match nftoken_mint_flag {
                NFTokenMintFlag::TfBurnable => flags.contains(&NFTokenMintFlag::TfBurnable),
                NFTokenMintFlag::TfOnlyXRP => flags.contains(&NFTokenMintFlag::TfOnlyXRP),
                NFTokenMintFlag::TfTransferable => flags.contains(&NFTokenMintFlag::TfTransferable),
                NFTokenMintFlag::TfTrustline => flags.contains(&NFTokenMintFlag::TfTrustline),
            },
            _ => false,
        }
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl NFTokenMintError for NFTokenMint<'static> {
    fn _get_issuer_error(&self) -> Result<(), NFTokenMintException> {
        match self.issuer {
            Some(issuer) => match issuer == self.account {
                true => Err(NFTokenMintException::InvalidIssuerMustNotEqualAccount),
                false => Ok(()),
            },
            None => Ok(()),
        }
    }

    fn _get_transfer_fee_error(&self) -> Result<(), NFTokenMintException> {
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

    fn _get_uri_error(&self) -> Result<(), NFTokenMintException> {
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::offer_cancel")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<u32>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the OfferCancel model.
    ///
    /// See OfferCancel fields:
    /// `<https://xrpl.org/offercancel.html#offercancel-fields>`
    offer_sequence: u32,
}

impl Model for OfferCancel<'static> {
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `OfferCancel` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        transaction_json
    }
}

impl From<&OfferCancel<'static>> for u32 {
    fn from(_: &OfferCancel<'static>) -> Self {
        0
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::offer_create")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<OfferCreateFlag>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
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
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `OfferCreate` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        if self.taker_gets.is_xrp() {
            transaction_json["TakerGets"] =
                Value::from(self.taker_gets.get_value_as_u32().to_string().as_str());
        }
        if self.taker_pays.is_xrp() {
            transaction_json["TakerPays"] =
                Value::from(self.taker_pays.get_value_as_u32().to_string().as_str());
        }
        transaction_json
    }
}

impl From<&OfferCreate<'static>> for u32 {
    fn from(val: &OfferCreate<'static>) -> Self {
        val.flags
            .as_ref()
            .unwrap_or(&Vec::new())
            .iter()
            .fold(0, |collect, flag| match flag {
                OfferCreateFlag::TfPassive => collect + 0x00010000,
                OfferCreateFlag::TfImmediateOrCancel => collect + 0x00020000,
                OfferCreateFlag::TfFillOrKill => collect + 0x00040000,
                OfferCreateFlag::TfSell => collect + 0x00080000,
            })
    }
}

impl Transaction for OfferCreate<'static> {
    fn has_flag(&self, flag: &Flag) -> bool {
        let mut flags = &Vec::new();

        if let Some(flag_set) = self.flags.as_ref() {
            flags = flag_set;
        }

        match flag {
            Flag::OfferCreate(offer_create_flag) => match offer_create_flag {
                OfferCreateFlag::TfFillOrKill => flags.contains(&OfferCreateFlag::TfFillOrKill),
                OfferCreateFlag::TfImmediateOrCancel => {
                    flags.contains(&OfferCreateFlag::TfImmediateOrCancel)
                }
                OfferCreateFlag::TfPassive => flags.contains(&OfferCreateFlag::TfPassive),
                OfferCreateFlag::TfSell => flags.contains(&OfferCreateFlag::TfSell),
            },
            _ => false,
        }
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::payment")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<PaymentFlag>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
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
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `Payment` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        if self.amount.is_xrp() {
            transaction_json["Amount"] =
                Value::from(self.amount.get_value_as_u32().to_string().as_str());
        }
        if let Some(send_max) = &self.send_max {
            if send_max.is_xrp() {
                transaction_json["SendMax"] =
                    Value::from(send_max.get_value_as_u32().to_string().as_str());
            }
        }
        if let Some(deliver_min) = &self.deliver_min {
            if deliver_min.is_xrp() {
                transaction_json["DeliverMin"] =
                    Value::from(deliver_min.get_value_as_u32().to_string().as_str());
            }
        }
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self._get_xrp_transaction_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::PaymentError(error),
            )),
            Ok(_no_error) => match self._get_partial_payment_error() {
                Err(error) => Err(XRPLModelException::XRPLTransactionError(
                    XRPLTransactionException::PaymentError(error),
                )),
                Ok(_no_error) => match self._get_exchange_error() {
                    Err(error) => Err(XRPLModelException::XRPLTransactionError(
                        XRPLTransactionException::PaymentError(error),
                    )),
                    Ok(_no_error) => Ok(()),
                },
            },
        }
    }
}

impl From<&Payment<'static>> for u32 {
    fn from(val: &Payment<'static>) -> Self {
        val.flags
            .as_ref()
            .unwrap_or(&Vec::new())
            .iter()
            .fold(0, |collect, flag| match flag {
                PaymentFlag::TfNoDirectRipple => collect + 0x00010000,
                PaymentFlag::TfPartialPayment => collect + 0x00020000,
                PaymentFlag::TfLimitQuality => collect + 0x00040000,
            })
    }
}

impl Transaction for Payment<'static> {
    fn has_flag(&self, flag: &Flag) -> bool {
        let mut flags = &Vec::new();

        if let Some(flag_set) = self.flags.as_ref() {
            flags = flag_set;
        }

        match flag {
            Flag::Payment(payment_flag) => match payment_flag {
                PaymentFlag::TfLimitQuality => flags.contains(&PaymentFlag::TfLimitQuality),
                PaymentFlag::TfNoDirectRipple => flags.contains(&PaymentFlag::TfNoDirectRipple),
                PaymentFlag::TfPartialPayment => flags.contains(&PaymentFlag::TfPartialPayment),
            },
            _ => false,
        }
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl PaymentError for Payment<'static> {
    fn _get_xrp_transaction_error(&self) -> Result<(), PaymentException> {
        match self.amount.is_xrp() && self.send_max.is_none() {
            true => match self.paths.is_some() {
                true => Err(PaymentException::InvalidXRPtoXRPPaymentsCannotContainPaths),
                false => match self.account == self.destination {
                    true => Err(
                        PaymentException::InvalidDestinationMustNotEqualAccountForXRPtoXRPPayments,
                    ),
                    false => Ok(()),
                },
            },
            false => Ok(()),
        }
    }

    fn _get_partial_payment_error(&self) -> Result<(), PaymentException> {
        match self.send_max.as_ref() {
            Some(send_max) => match !self.has_flag(&Flag::Payment(PaymentFlag::TfPartialPayment)) {
                true => match send_max.is_xrp() && self.amount.is_xrp() {
                    true => Err(
                        PaymentException::InvalidSendMaxMustNotBeSetForXRPtoXRPNonPartialPayments,
                    ),
                    false => Ok(()),
                },
                false => Ok(()),
            },
            None => match self.has_flag(&Flag::Payment(PaymentFlag::TfPartialPayment)) {
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

    fn _get_exchange_error(&self) -> Result<(), PaymentException> {
        match self.account == self.destination {
            true => match self.send_max.as_ref() {
                Some(_send_max) => Ok(()),
                None => Err(PaymentException::InvalidSendMaxMustBeSetForExchanges),
            },
            false => Ok(()),
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::payment_channel_claim")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<PaymentChannelClaimFlag>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
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
    fn to_json_value(&self) -> Value {
        let mut transaction_json = serde_json::to_value(&self)
            .expect("Unable to serialize `PaymentChannelClaim` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        transaction_json
    }
}

impl From<&PaymentChannelClaim<'static>> for u32 {
    fn from(val: &PaymentChannelClaim<'static>) -> Self {
        val.flags
            .as_ref()
            .unwrap_or(&Vec::new())
            .iter()
            .fold(0, |collect, flag| match flag {
                PaymentChannelClaimFlag::TfRenew => collect + 0x00010000,
                PaymentChannelClaimFlag::TfClose => collect + 0x00020000,
            })
    }
}

impl Transaction for PaymentChannelClaim<'static> {
    fn has_flag(&self, flag: &Flag) -> bool {
        let mut flags = &Vec::new();

        if let Some(flag_set) = self.flags.as_ref() {
            flags = flag_set;
        }

        match flag {
            Flag::PaymentChannelClaim(payment_channel_claim_flag) => {
                match payment_channel_claim_flag {
                    PaymentChannelClaimFlag::TfClose => {
                        flags.contains(&PaymentChannelClaimFlag::TfClose)
                    }
                    PaymentChannelClaimFlag::TfRenew => {
                        flags.contains(&PaymentChannelClaimFlag::TfRenew)
                    }
                }
            }
            _ => false,
        }
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::payment_channel_create")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<u32>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
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
    fn to_json_value(&self) -> Value {
        let mut transaction_json = serde_json::to_value(&self)
            .expect("Unable to serialize `PaymentChannelCreate` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        if self.amount.is_xrp() {
            transaction_json["Amount"] =
                Value::from(self.amount.get_value_as_u32().to_string().as_str());
        }
        transaction_json
    }
}

impl From<&PaymentChannelCreate<'static>> for u32 {
    fn from(_: &PaymentChannelCreate<'static>) -> Self {
        0
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::payment_channel_fund")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<u32>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
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
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `PaymentChannelFund` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        transaction_json
    }
}

impl From<&PaymentChannelFund<'static>> for u32 {
    fn from(_: &PaymentChannelFund<'static>) -> Self {
        0
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::set_regular_key")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<u32>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the SetRegularKey model.
    ///
    /// See SetRegularKey fields:
    /// `<https://xrpl.org/setregularkey.html#setregularkey-fields>`
    regular_key: Option<&'a str>,
}

impl Model for SetRegularKey<'static> {
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `SetRegularKey` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        transaction_json
    }
}

impl From<&SetRegularKey<'static>> for u32 {
    fn from(_: &SetRegularKey<'static>) -> Self {
        0
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::signer_list_set")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<u32>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the TicketCreate model.
    ///
    /// See TicketCreate fields:
    /// `<https://xrpl.org/signerlistset.html#signerlistset-fields>`
    signer_quorum: u32,
    signer_entries: Option<Vec<SignerEntry<'a>>>,
}

impl Model for SignerListSet<'static> {
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `SignerListSet` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self._get_signer_entries_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::SignerListSetError(error),
            )),
            Ok(_no_error) => match self._get_signer_quorum_error() {
                Err(error) => Err(XRPLModelException::XRPLTransactionError(
                    XRPLTransactionException::SignerListSetError(error),
                )),
                Ok(_no_error) => Ok(()),
            },
        }
    }
}

impl From<&SignerListSet<'static>> for u32 {
    fn from(_: &SignerListSet<'static>) -> Self {
        0
    }
}

impl Transaction for SignerListSet<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl SignerListSetError for SignerListSet<'static> {
    fn _get_signer_entries_error(&self) -> Result<(), SignerListSetException> {
        match self.signer_entries.as_ref() {
            Some(signer_entries) => match self.signer_quorum == 0 {
                true => Err(SignerListSetException::InvalidMustNotSetSignerEntriesIfSignerListIsBeingDeleted),
                false => match signer_entries.is_empty() {
                    true => Err(SignerListSetException::InvalidTooFewSignerEntries { min: 1, found: signer_entries.len() }),
                    false => match signer_entries.len() > 8 {
                        true => Err(SignerListSetException::InvalidTooManySignerEntries { max: 8, found: signer_entries.len() }),
                        false => Ok(())
                    },
                },
            },
            None => Ok(())
        }
    }

    fn _get_signer_quorum_error(&self) -> Result<(), SignerListSetException> {
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
        let mut check_account = Vec::new();
        for account in accounts.clone() {
            match &check_account.contains(&account) {
                true => {
                    return Err(
                        SignerListSetException::InvalidAnAccountCanNotBeInSignerEntriesTwice,
                    )
                }
                false => check_account.push(account),
            }
        }
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::ticket_create")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<u32>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the TicketCreate model.
    ///
    /// See TicketCreate fields:
    /// `<https://xrpl.org/ticketcreate.html#ticketcreate-fields>`
    ticket_count: u32,
}

impl Model for TicketCreate<'static> {
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `TicketCreate` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        transaction_json
    }
}

impl From<&TicketCreate<'static>> for u32 {
    fn from(_: &TicketCreate<'static>) -> Self {
        0
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::trust_set")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<TrustSetFlag>>,
    /// Additional arbitrary information used to identify this transaction.
    memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
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
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `TrustSet` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        if self.limit_amount.is_xrp() {
            transaction_json["LimitAmount"] =
                Value::from(self.limit_amount.get_value_as_u32().to_string().as_str());
        }
        transaction_json
    }
}

impl From<&TrustSet<'static>> for u32 {
    fn from(val: &TrustSet<'static>) -> Self {
        val.flags
            .as_ref()
            .unwrap_or(&Vec::new())
            .iter()
            .fold(0, |collect, flag| match flag {
                TrustSetFlag::TfSetAuth => collect + 0x00010000,
                TrustSetFlag::TfSetNoRipple => collect + 0x00020000,
                TrustSetFlag::TfClearNoRipple => collect + 0x00040000,
                TrustSetFlag::TfSetFreeze => collect + 0x00100000,
                TrustSetFlag::TfClearFreeze => collect + 0x00200000,
            })
    }
}

impl Transaction for TrustSet<'static> {
    fn has_flag(&self, flag: &Flag) -> bool {
        let mut flags = &Vec::new();

        if let Some(flag_set) = self.flags.as_ref() {
            flags = flag_set;
        }

        match flag {
            Flag::TrustSet(trust_set_flag) => match trust_set_flag {
                TrustSetFlag::TfClearFreeze => flags.contains(&TrustSetFlag::TfClearFreeze),
                TrustSetFlag::TfClearNoRipple => flags.contains(&TrustSetFlag::TfClearNoRipple),
                TrustSetFlag::TfSetAuth => flags.contains(&TrustSetFlag::TfSetAuth),
                TrustSetFlag::TfSetFreeze => flags.contains(&TrustSetFlag::TfSetFreeze),
                TrustSetFlag::TfSetNoRipple => flags.contains(&TrustSetFlag::TfSetNoRipple),
            },
            _ => false,
        }
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::enable_amendment")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<EnableAmendmentFlag>>,
    /// The custom fields for the EnableAmendment model.
    ///
    /// See EnableAmendment fields:
    /// `<https://xrpl.org/enableamendment.html#enableamendment-fields>`
    amendment: &'a str,
    ledger_sequence: u32,
}

impl Model for EnableAmendment<'static> {
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `EnableAmendment` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        transaction_json
    }
}

impl From<&EnableAmendment<'static>> for u32 {
    fn from(val: &EnableAmendment<'static>) -> Self {
        val.flags
            .as_ref()
            .unwrap_or(&Vec::new())
            .iter()
            .fold(0, |collect, flag| match flag {
                EnableAmendmentFlag::TfGotMajority => collect + 0x00010000,
                EnableAmendmentFlag::TfLostMajority => collect + 0x00020000,
            })
    }
}

impl Transaction for EnableAmendment<'static> {
    fn has_flag(&self, flag: &Flag) -> bool {
        match flag {
            Flag::EnableAmendment(enable_amendment_flag) => match enable_amendment_flag {
                EnableAmendmentFlag::TfGotMajority => self
                    .flags
                    .as_ref()
                    .unwrap()
                    .contains(&EnableAmendmentFlag::TfGotMajority),
                EnableAmendmentFlag::TfLostMajority => self
                    .flags
                    .as_ref()
                    .unwrap()
                    .contains(&EnableAmendmentFlag::TfLostMajority),
            },
            _ => false,
        }
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::set_fee")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
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
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `SetFee` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        transaction_json
    }
}

impl From<&SetFee<'static>> for u32 {
    fn from(_: &SetFee<'static>) -> Self {
        0
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
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::unl_modify")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    #[serde(default = "default_account_zero")]
    account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    sequence: Option<u32>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    source_tag: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    flags: Option<Vec<u32>>,
    /// The custom fields for the UNLModify model.
    ///
    /// See UNLModify fields:
    /// `<https://xrpl.org/unlmodify.html#unlmodify-fields>`
    ledger_sequence: u32,
    unlmodify_disabling: u8,
    unlmodify_validator: &'a str,
}

impl Model for UNLModify<'static> {
    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `UNLModify` to json.");
        transaction_json["Flags"] = Value::from(u32::from(self));
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self._get_unl_modify_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::UNLModifyError(error),
            )),
            Ok(_no_error) => Ok(()),
        }
    }
}

impl From<&UNLModify<'static>> for u32 {
    fn from(_: &UNLModify<'static>) -> Self {
        0
    }
}

impl Transaction for UNLModify<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl UNLModifyError for UNLModify<'static> {
    fn _get_unl_modify_error(&self) -> Result<(), UNLModifyException> {
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
        let json = r#"{"Account":"rpXhhWmCvDwkzNtRbm7mmD1vZqdfatQNEe","Fee":"10","Flags":131072,"LastLedgerSequence":72779837,"Sequence":1,"TakerGets":"1000000","TakerPays":{"currency":"USD","issuer":"rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq","value":"0.3"},"TransactionType":{"transaction_type":"OfferCreate"}}"#;
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
        assert!(offer_create.has_flag(&Flag::OfferCreate(OfferCreateFlag::TfImmediateOrCancel)));
        assert!(!offer_create.has_flag(&Flag::OfferCreate(OfferCreateFlag::TfPassive)));
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
        assert_eq!(account_set.validate(), Err(expected_error));

        let tick_size_too_high = Some(16);
        account_set.tick_size = tick_size_too_high;
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::AccountSetError(
                AccountSetException::InvalidTickSizeTooHigh { max: 15, found: 16 },
            ));
        assert_eq!(account_set.validate(), Err(expected_error));
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
        assert_eq!(account_set.validate(), Err(expected_error));

        let tick_size_too_high = Some(2000000001);
        account_set.transfer_rate = tick_size_too_high;
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::AccountSetError(
                AccountSetException::InvalidTransferRateTooHigh {
                    max: 2000000000,
                    found: 2000000001,
                },
            ));
        assert_eq!(account_set.validate(), Err(expected_error));
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
        assert_eq!(account_set.validate(), Err(expected_error));

        let domain_too_long = Some("https://example.com/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        account_set.domain = domain_too_long;
        let expected_error = XRPLModelException::XRPLTransactionError(
            XRPLTransactionException::AccountSetError(AccountSetException::InvalidDomainTooLong {
                max: 256,
                found: 270,
            }),
        );
        assert_eq!(account_set.validate(), Err(expected_error));
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
        assert_eq!(account_set.validate(), Err(expected_error));
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
        assert_eq!(account_set.validate(), Err(expected_error));

        account_set.nftoken_minter = None;
        account_set.set_flag = Some(AccountSetFlag::AsfAuthorizedNFTokenMinter);
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::AccountSetError(
                AccountSetException::InvalidNftokenMinterMustBeSetIfAsfAuthorizedNftokenMinterIsSet,
            ));
        assert_eq!(account_set.validate(), Err(expected_error));

        account_set.set_flag = None;
        account_set.nftoken_minter = Some("rLSn6Z3T8uCxbcd1oxwfGQN1Fdn5CyGujK");
        account_set.clear_flag = Some(AccountSetFlag::AsfAuthorizedNFTokenMinter);
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::AccountSetError(
                AccountSetException::InvalidNftokenMinterMustNotBeSetIfAsfAuthorizedNftokenMinterIsUnset,
            ));
        assert_eq!(account_set.validate(), Err(expected_error));
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
        assert_eq!(check_cash.validate(), Err(expected_error));

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
        assert_eq!(check_cash.validate(), Err(expected_error));
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
        assert_eq!(deposit_preauth.validate(), Err(expected_error));

        deposit_preauth.authorize = Some("rLSn6Z3T8uCxbcd1oxwfGQN1Fdn5CyGujK");
        deposit_preauth.unauthorize = Some("raQwCVAJVqjrVm1Nj5SFRcX8i22BhdC9WA");
        let expected_error = XRPLModelException::XRPLTransactionError(
            XRPLTransactionException::DepositPreauthError(
                DepositPreauthException::InvalidMustNotSetAuthorizeAndUnauthorize,
            ),
        );
        assert_eq!(deposit_preauth.validate(), Err(expected_error));
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
        assert_eq!(escrow_create.validate(), Err(expected_error));
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
        assert_eq!(escrow_finish.validate(), Err(expected_error));
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
        assert_eq!(nftoken_accept_offer.validate(), Err(expected_error));
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
        assert_eq!(nftoken_accept_offer.validate(), Err(expected_error));
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
        assert_eq!(nftoken_cancel_offer.validate(), Err(expected_error));
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
        assert_eq!(nftoken_create_offer.validate(), Err(expected_error));
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
        assert_eq!(nftoken_create_offer.validate(), Err(expected_error));
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
        assert_eq!(nftoken_create_offer.validate(), Err(expected_error));

        nftoken_create_offer.flags = None;
        nftoken_create_offer.owner = None;
        let expected_error = XRPLModelException::XRPLTransactionError(
            XRPLTransactionException::NFTokenCreateOfferError(
                NFTokenCreateOfferException::InvalidOwnerMustBeSetForBuyOffer,
            ),
        );
        assert_eq!(nftoken_create_offer.validate(), Err(expected_error));

        nftoken_create_offer.owner = Some("rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb");
        let expected_error = XRPLModelException::XRPLTransactionError(
            XRPLTransactionException::NFTokenCreateOfferError(
                NFTokenCreateOfferException::InvalidOwnerMustNotEqualAccount,
            ),
        );
        assert_eq!(nftoken_create_offer.validate(), Err(expected_error));
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
        assert_eq!(nftoken_mint.validate(), Err(expected_error));
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
        assert_eq!(nftoken_mint.validate(), Err(expected_error));
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
        assert_eq!(nftoken_mint.validate(), Err(expected_error));
    }
}

#[cfg(test)]
mod test_payment_error {
    use alloc::{borrow::Cow, vec};

    use crate::models::{
        exceptions::{PaymentException, XRPLModelException, XRPLTransactionException},
        Currency, Model, PathStep, PaymentFlag, TransactionType,
    };

    use super::Payment;

    #[test]
    fn test_xrp_to_xrp_error() {
        let mut payment = Payment {
            transaction_type: TransactionType::Payment,
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
                value: Some(Cow::Borrowed("1000000")),
                currency: Cow::Borrowed("XRP"),
            },
            destination: "rLSn6Z3T8uCxbcd1oxwfGQN1Fdn5CyGujK",
            destination_tag: None,
            invoice_id: None,
            paths: Some(vec![vec![PathStep {
                account: Some("rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B"),
                currency: None,
                issuer: None,
                r#type: None,
                type_hex: None,
            }]]),
            send_max: None,
            deliver_min: None,
        };
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::PaymentError(
                PaymentException::InvalidXRPtoXRPPaymentsCannotContainPaths,
            ));
        assert_eq!(payment.validate(), Err(expected_error));

        payment.paths = None;
        payment.send_max = Some(Currency::Xrp {
            value: Some(Cow::Borrowed("99999")),
            currency: Cow::Borrowed("XRP"),
        });
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::PaymentError(
                PaymentException::InvalidSendMaxMustNotBeSetForXRPtoXRPNonPartialPayments,
            ));
        assert_eq!(payment.validate(), Err(expected_error));

        payment.send_max = None;
        payment.destination = "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb";
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::PaymentError(
                PaymentException::InvalidDestinationMustNotEqualAccountForXRPtoXRPPayments,
            ));
        assert_eq!(payment.validate(), Err(expected_error));
    }

    #[test]
    fn test_partial_payments_eror() {
        let mut payment = Payment {
            transaction_type: TransactionType::Payment,
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
                value: Some(Cow::Borrowed("1000000")),
                currency: Cow::Borrowed("XRP"),
            },
            destination: "rLSn6Z3T8uCxbcd1oxwfGQN1Fdn5CyGujK",
            destination_tag: None,
            invoice_id: None,
            paths: None,
            send_max: None,
            deliver_min: None,
        };
        payment.flags = Some(vec![PaymentFlag::TfPartialPayment]);
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::PaymentError(
                PaymentException::InvalidSendMaxMustBeSetForPartialPayments,
            ));
        assert_eq!(payment.validate(), Err(expected_error));

        payment.flags = None;
        payment.deliver_min = Some(Currency::Xrp {
            value: Some(Cow::Borrowed("900000")),
            currency: Cow::Borrowed("XRP"),
        });
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::PaymentError(
                PaymentException::InvalidDeliverMinMustNotBeSetForNonPartialPayments,
            ));
        assert_eq!(payment.validate(), Err(expected_error));
    }

    #[test]
    fn test_exchange_error() {
        let payment = Payment {
            transaction_type: TransactionType::Payment,
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
            amount: Currency::IssuedCurrency {
                value: Some(Cow::Borrowed("10")),
                currency: Cow::Borrowed("USD"),
                issuer: Cow::Borrowed("rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B"),
            },
            destination: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            destination_tag: None,
            invoice_id: None,
            paths: None,
            send_max: None,
            deliver_min: None,
        };
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::PaymentError(
                PaymentException::InvalidSendMaxMustBeSetForExchanges,
            ));
        assert_eq!(payment.validate(), Err(expected_error));
    }
}

#[cfg(test)]
mod test_signer_list_set_error {
    use alloc::vec;

    use crate::models::{
        exceptions::{SignerListSetException, XRPLModelException, XRPLTransactionException},
        Model, SignerEntry, TransactionType,
    };

    use super::SignerListSet;

    #[test]
    fn test_signer_list_deleted_error() {
        let mut signer_list_set = SignerListSet {
            transaction_type: TransactionType::SignerListSet,
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
            signer_quorum: 0,
            signer_entries: Some(vec![SignerEntry {
                account: "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW",
                signer_weight: 2,
            }]),
        };
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::SignerListSetError(
                SignerListSetException::InvalidMustNotSetSignerEntriesIfSignerListIsBeingDeleted,
            ));
        assert_eq!(signer_list_set.validate(), Err(expected_error));

        signer_list_set.signer_quorum = 3;
        signer_list_set.signer_entries = None;
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::SignerListSetError(
                SignerListSetException::InvalidSignerQuorumMustBeZeroIfSignerListIsBeingDeleted,
            ));
        assert_eq!(signer_list_set.validate(), Err(expected_error));
    }

    #[test]
    fn test_signer_entries_error() {
        let mut signer_list_set = SignerListSet {
            transaction_type: TransactionType::SignerListSet,
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
            signer_quorum: 3,
            signer_entries: Some(vec![]),
        };
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::SignerListSetError(
                SignerListSetException::InvalidTooFewSignerEntries { min: 1, found: 0 },
            ));
        assert_eq!(signer_list_set.validate(), Err(expected_error));

        signer_list_set.signer_entries = Some(vec![
            SignerEntry {
                account: "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW",
                signer_weight: 1,
            },
            SignerEntry {
                account: "rUpy3eEg8rqjqfUoLeBnZkscbKbFsKXC3v",
                signer_weight: 1,
            },
            SignerEntry {
                account: "rUpy3eEg8rqjqfUoLeBnZkscbKbFsKXC3v",
                signer_weight: 2,
            },
            SignerEntry {
                account: "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
                signer_weight: 2,
            },
            SignerEntry {
                account: "rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B",
                signer_weight: 1,
            },
            SignerEntry {
                account: "rXTZ5g8X7mrAYEe7iFeM9fiS4ccueyurG",
                signer_weight: 1,
            },
            SignerEntry {
                account: "rPbMHxs7vy5t6e19tYfqG7XJ6Fog8EPZLk",
                signer_weight: 2,
            },
            SignerEntry {
                account: "r3rhWeE31Jt5sWmi4QiGLMZnY3ENgqw96W",
                signer_weight: 3,
            },
            SignerEntry {
                account: "rchGBxcD1A1C2tdxF6papQYZ8kjRKMYcL",
                signer_weight: 2,
            },
        ]);
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::SignerListSetError(
                SignerListSetException::InvalidTooManySignerEntries { max: 8, found: 9 },
            ));
        assert_eq!(signer_list_set.validate(), Err(expected_error));

        signer_list_set.signer_entries = Some(vec![
            SignerEntry {
                account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
                signer_weight: 1,
            },
            SignerEntry {
                account: "rUpy3eEg8rqjqfUoLeBnZkscbKbFsKXC3v",
                signer_weight: 2,
            },
            SignerEntry {
                account: "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
                signer_weight: 2,
            },
        ]);
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::SignerListSetError(
                SignerListSetException::InvalidAccountMustNotBeInSignerEntry,
            ));
        assert_eq!(signer_list_set.validate(), Err(expected_error));

        signer_list_set.signer_entries = Some(vec![SignerEntry {
            account: "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW",
            signer_weight: 3,
        }]);
        signer_list_set.signer_quorum = 10;
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::SignerListSetError(
                SignerListSetException::InvalidMustBeLessOrEqualToSumOfSignerWeightInSignerEntries { max: 3, found: 10 },
            ));
        assert_eq!(signer_list_set.validate(), Err(expected_error));

        signer_list_set.signer_entries = Some(vec![
            SignerEntry {
                account: "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW",
                signer_weight: 3,
            },
            SignerEntry {
                account: "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW",
                signer_weight: 2,
            },
        ]);
        signer_list_set.signer_quorum = 2;
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::SignerListSetError(
                SignerListSetException::InvalidAnAccountCanNotBeInSignerEntriesTwice,
            ));
        assert_eq!(signer_list_set.validate(), Err(expected_error));
    }
}

#[cfg(test)]
mod test_unl_modify_error {
    use crate::models::{
        exceptions::{UNLModifyException, XRPLModelException, XRPLTransactionException},
        Model, TransactionType,
    };

    use super::UNLModify;

    #[test]
    fn test_unlmodify_disabling_error() {
        let unl_modify = UNLModify {
            transaction_type: TransactionType::UNLModify,
            account: "",
            fee: None,
            sequence: None,
            signing_pub_key: None,
            source_tag: None,
            txn_signature: None,
            flags: None,
            ledger_sequence: 1600000,
            unlmodify_disabling: 3,
            unlmodify_validator:
                "ED6629D456285AE3613B285F65BBFF168D695BA3921F309949AFCD2CA7AFEC16FE",
        };
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::UNLModifyError(
                UNLModifyException::InvalidUNLModifyDisablingMustBeOneOrTwo,
            ));
        assert_eq!(unl_modify.validate(), Err(expected_error));
    }
}
