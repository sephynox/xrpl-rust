use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::{AsRefStr, Display};

use crate::{
    constants::{
        DISABLE_TICK_SIZE, MAX_DOMAIN_LENGTH, MAX_TICK_SIZE, MAX_TRANSFER_RATE, MIN_TICK_SIZE,
        MIN_TRANSFER_RATE, SPECIAL_CASE_TRANFER_RATE,
    },
    models::{
        exceptions::{AccountSetException, XRPLModelException, XRPLTransactionException},
        model::Model,
        AccountSetError, Flag, Memo, Signer, Transaction, TransactionType,
    },
};

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

/// An AccountSet transaction modifies the properties of an
/// account in the XRP Ledger.
///
/// See AccountSet:
/// `<https://xrpl.org/accountset.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
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
    pub transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    pub account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    pub fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    pub sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    pub last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    pub account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    pub signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    pub source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    pub ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    pub txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    pub flags: Option<Vec<AccountSetFlag>>,
    /// Additional arbitrary information used to identify this transaction.
    pub memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    pub signers: Option<Vec<Signer<'a>>>,
    // The custom fields for the AccountSet model.
    //
    // See AccountSet fields:
    // `<https://xrpl.org/accountset.html#accountset-fields>`
    /// Unique identifier of a flag to disable for this account.
    pub clear_flag: Option<AccountSetFlag>,
    /// The domain that owns this account, as a string of hex
    /// representing the ASCII for the domain in lowercase.
    /// Cannot be more than 256 bytes in length.
    pub domain: Option<&'a str>,
    /// Hash of an email address to be used for generating an
    /// avatar image. Conventionally, clients use Gravatar
    /// to display this image.
    pub email_hash: Option<&'a str>,
    /// Public key for sending encrypted messages to this account.
    /// To set the key, it must be exactly 33 bytes, with the
    /// first byte indicating the key type: 0x02 or 0x03 for
    /// secp256k1 keys, 0xED for Ed25519 keys. To remove the
    /// key, use an empty value.
    pub message_key: Option<&'a str>,
    /// Sets an alternate account that is allowed to mint NFTokens
    /// on this account's behalf using NFTokenMint's Issuer field.
    /// This field is part of the experimental XLS-20 standard
    /// for non-fungible tokens.
    pub nftoken_minter: Option<&'a str>,
    /// Flag to enable for this account.
    pub set_flag: Option<AccountSetFlag>,
    /// The fee to charge when users transfer this account's tokens,
    /// represented as billionths of a unit. Cannot be more than
    /// 2000000000 or less than 1000000000, except for the special
    /// case 0 meaning no fee.
    pub transfer_rate: Option<u32>,
    /// Tick size to use for offers involving a currency issued by
    /// this address. The exchange rates of those offers is rounded
    /// to this many significant digits. Valid values are 3 to 15
    /// inclusive, or 0 to disable.
    pub tick_size: Option<u32>,
}

impl<'a> Default for AccountSet<'a> {
    fn default() -> Self {
        Self {
            transaction_type: TransactionType::AccountSet,
            account: Default::default(),
            fee: Default::default(),
            sequence: Default::default(),
            last_ledger_sequence: Default::default(),
            account_txn_id: Default::default(),
            signing_pub_key: Default::default(),
            source_tag: Default::default(),
            ticket_sequence: Default::default(),
            txn_signature: Default::default(),
            flags: Default::default(),
            memos: Default::default(),
            signers: Default::default(),
            clear_flag: Default::default(),
            domain: Default::default(),
            email_hash: Default::default(),
            message_key: Default::default(),
            nftoken_minter: Default::default(),
            set_flag: Default::default(),
            transfer_rate: Default::default(),
            tick_size: Default::default(),
        }
    }
}

impl<'a> Model for AccountSet<'a> {
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

impl<'a> Transaction for AccountSet<'a> {
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

impl<'a> AccountSetError for AccountSet<'a> {
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

impl<'a> AccountSet<'a> {
    fn new(
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
        clear_flag: Option<AccountSetFlag>,
        domain: Option<&'a str>,
        email_hash: Option<&'a str>,
        message_key: Option<&'a str>,
        set_flag: Option<AccountSetFlag>,
        transfer_rate: Option<u32>,
        tick_size: Option<u32>,
        nftoken_minter: Option<&'a str>,
    ) -> Self {
        Self {
            transaction_type: TransactionType::AccountSet,
            account,
            fee,
            sequence,
            last_ledger_sequence,
            account_txn_id,
            signing_pub_key,
            source_tag,
            ticket_sequence,
            txn_signature,
            flags,
            memos,
            signers,
            clear_flag,
            domain,
            email_hash,
            message_key,
            nftoken_minter,
            set_flag,
            transfer_rate,
            tick_size,
        }
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
