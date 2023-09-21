pub mod exceptions;
use alloc::{borrow::Cow, string::String};
use anyhow::Result;
use derive_new::new;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    asynch::transaction::exceptions::XRPLSignTransactionException,
    core::{
        addresscodec::{is_valid_xaddress, xaddress_to_classic_address},
        binarycodec::encode_for_signing,
        keypairs::sign as keypairs_sign,
    },
    models::transactions::{Transaction, XRPLTransactionFieldException},
    utils::transactions::{
        get_transaction_field_value, set_transaction_field_value, validate_transaction_has_field,
    },
    wallet::Wallet,
    Err,
};

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, new)]
#[serde(rename_all = "PascalCase")]
pub struct PreparedTransaction<'a, T> {
    #[serde(flatten)]
    transaction: T,
    signing_pub_key: Cow<'a, str>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, new)]
#[serde(rename_all = "PascalCase")]
pub struct SignedTransaction<'a, T> {
    #[serde(flatten)]
    prepared_transaction: PreparedTransaction<'a, T>,
    signature: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
enum AccountFieldType {
    Account,
    Destination,
}

pub fn sign<T>(
    transaction: T,
    wallet: &Wallet,
    _multisign: bool,
) -> Result<SignedTransaction<'_, T>>
where
    T: Transaction + Serialize + DeserializeOwned + 'static + Clone,
{
    let prepared_transaction = prepare_transaction(transaction, wallet)?;
    let serialized_for_signing = encode_for_signing(&prepared_transaction)?;
    let serialized_bytes = hex::decode(serialized_for_signing).unwrap(); // TODO: handle unwrap
    let signature = keypairs_sign(&serialized_bytes, &wallet.private_key).unwrap(); // TODO: handle unwrap
    let signed_transaction = SignedTransaction::new(prepared_transaction, signature.into());

    Ok(signed_transaction)
}

fn prepare_transaction<T>(transaction: T, wallet: &Wallet) -> Result<PreparedTransaction<'_, T>>
where
    T: Transaction + Serialize + DeserializeOwned + Clone,
{
    let mut prepared_transaction =
        PreparedTransaction::new(transaction, Cow::from(wallet.classic_address.clone()));

    prepared_transaction =
        validate_account_xaddress(prepared_transaction, AccountFieldType::Account)?;
    if validate_transaction_has_field(&prepared_transaction.transaction, "Destination").is_ok() {
        prepared_transaction =
            validate_account_xaddress(prepared_transaction, AccountFieldType::Destination)?;
    }

    prepared_transaction.transaction =
        convert_to_classic_address(&prepared_transaction.transaction, "Unauthorize")
            .unwrap_or(prepared_transaction.transaction);
    prepared_transaction.transaction =
        convert_to_classic_address(&prepared_transaction.transaction, "Authorize")
            .unwrap_or(prepared_transaction.transaction);
    // EscrowCancel, EscrowFinish
    prepared_transaction.transaction =
        convert_to_classic_address(&prepared_transaction.transaction, "Owner")
            .unwrap_or(prepared_transaction.transaction);
    // SetRegularKey
    prepared_transaction.transaction =
        convert_to_classic_address(&prepared_transaction.transaction, "RegularKey")
            .unwrap_or(prepared_transaction.transaction);

    Ok(prepared_transaction)
}

fn validate_account_xaddress<T>(
    mut prepared_transaction: PreparedTransaction<'_, T>,
    account_field: AccountFieldType,
) -> Result<PreparedTransaction<'_, T>>
where
    T: Transaction + Serialize + DeserializeOwned + Clone,
{
    let (account_field_name, tag_field_name) = match serde_json::to_string(&account_field) {
        Ok(name) => {
            let name_str = name.as_str().trim();
            if name_str == "\"Account\"" {
                ("Account", "SourceTag")
            } else if name_str == "\"Destination\"" {
                ("Destination", "DestinationTag")
            } else {
                return Err!(XRPLTransactionFieldException::UnknownAccountField(name_str));
            }
        }
        Err(error) => return Err!(error),
    };
    let account_address = get_transaction_field_value::<_, String>(
        &prepared_transaction.transaction,
        &account_field_name,
    )?;

    if is_valid_xaddress(account_address.as_str()) {
        let (address, tag, _) = match xaddress_to_classic_address(account_address.as_str()) {
            Ok(t) => t,
            Err(error) => return Err!(error),
        };
        validate_transaction_has_field(&prepared_transaction.transaction, &account_field_name)?;
        prepared_transaction.transaction = set_transaction_field_value(
            &prepared_transaction.transaction,
            &account_field_name,
            address,
        )?;

        if validate_transaction_has_field(&prepared_transaction.transaction, &tag_field_name)
            .is_ok()
            && get_transaction_field_value(&prepared_transaction.transaction, &tag_field_name)
                .unwrap_or(Some(0))
                != tag
        {
            Err!(XRPLSignTransactionException::TagFieldMismatch(
                &tag_field_name
            ))
        } else {
            prepared_transaction.transaction = set_transaction_field_value(
                &prepared_transaction.transaction,
                &tag_field_name,
                tag,
            )?;

            Ok(prepared_transaction)
        }
    } else {
        Ok(prepared_transaction)
    }
}

fn convert_to_classic_address<T>(transaction: &T, field_name: &str) -> Result<T>
where
    T: Transaction + Serialize + DeserializeOwned + Clone,
{
    let address = get_transaction_field_value::<_, String>(transaction, field_name)?;
    if is_valid_xaddress(&address) {
        let classic_address = match xaddress_to_classic_address(&address) {
            Ok(t) => t.0,
            Err(error) => return Err!(error),
        };
        set_transaction_field_value(transaction, field_name, classic_address)
    } else {
        Ok(transaction.clone())
    }
}

#[cfg(test)]
mod test_sign {
    use alloc::borrow::Cow;

    use crate::{
        models::{amount::XRPAmount, transactions::Payment},
        wallet::Wallet,
    };

    #[test]
    fn test_sign() {
        let wallet = Wallet::create(None).unwrap();
        let payment = Payment::new(
            Cow::from(wallet.classic_address.clone()),
            XRPAmount::from("1000").into(),
            "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn".into(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        let signed_transaction = super::sign(payment, &wallet, false);
        assert!(signed_transaction.is_ok());
    }
}
