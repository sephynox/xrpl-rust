use core::any::Any;
use core::convert::TryInto;

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use anyhow::Result;
use rust_decimal::Decimal;
use serde::Serialize;
use strum::IntoEnumIterator;

use crate::models::amount::XRPAmount;
use crate::models::exceptions::XRPLModelException;
use crate::models::requests::ServerState;
use crate::models::results::server_state::ServerState as ServerStateResult;
use crate::models::transactions::AutofilledTransaction;
use crate::models::transactions::EscrowFinish;
use crate::models::transactions::Transaction;
use crate::models::transactions::TransactionType;
use crate::models::Model;
use crate::Err;

use self::exceptions::XRPLTransactionException;

use super::account::get_next_valid_seq_number;
use super::clients::Client;
use super::ledger::get_fee;
use super::ledger::get_latest_validated_ledger_sequence;

pub mod exceptions;

const OWNER_RESERVE: &'static str = "2000000"; // 2 XRP
const RESTRICTED_NETWORKS: u16 = 1024;
const REQUIRED_NETWORKID_VERSION: &'static str = "1.11.0";
const LEDGER_OFFSET: u8 = 20;

pub async fn autofill<'a, F, T>(
    transaction: T,
    client: &'a mut impl Client<'a>,
    signers_count: Option<u8>,
) -> Result<AutofilledTransaction<T>>
where
    T: Transaction<'a, F> + Model + 'static,
    F: IntoEnumIterator + Serialize + core::fmt::Debug + PartialEq + 'a,
{
    let mut autofilled_txn = AutofilledTransaction::new(transaction, None);
    set_network_id_and_build_version(client).await.unwrap();
    if autofilled_txn.netork_id.is_none() && txn_needs_network_id(client) {
        autofilled_txn.netork_id = client.get_common_fields().unwrap().network_id;
    }
    let txn_common_fields = autofilled_txn.transaction.get_mut_common_fields();
    if txn_common_fields.sequence.is_none() {
        txn_common_fields.sequence =
            Some(get_next_valid_seq_number(txn_common_fields.account.clone(), client, None).await?);
    }
    if txn_common_fields.fee.is_none() {
        txn_common_fields.fee = Some(
            calculate_fee_per_transaction_type(
                &autofilled_txn.transaction,
                Some(client),
                signers_count,
            )
            .await?,
        );
    }
    if txn_common_fields.last_ledger_sequence.is_none() {
        let ledger_sequence = get_latest_validated_ledger_sequence(client).await?;
        txn_common_fields.last_ledger_sequence = Some(ledger_sequence + LEDGER_OFFSET as u32);
    }

    Ok(autofilled_txn)
}

async fn check_fee<'a, F, T>(
    transaction: &'a T,
    client: Option<&'a mut impl Client<'a>>,
    signers_count: Option<u8>,
) -> Result<()>
where
    T: Transaction<'a, F> + Model + 'static,
    F: IntoEnumIterator + Serialize + core::fmt::Debug + PartialEq + 'a,
{
    let fee_to_high = XRPAmount::from("1000");
    let calculated_fee = calculate_fee_per_transaction_type(transaction, client, signers_count)
        .await
        .unwrap();
    let expected_fee = fee_to_high.max(calculated_fee);
    let common_fields = transaction.get_common_fields();
    if let Some(fee) = &common_fields.fee {
        if fee < &expected_fee {
            return Err!(XRPLTransactionException::FeeUnusuallyHigh(fee.clone()));
        }
    }

    Ok(())
}

async fn calculate_fee_per_transaction_type<'a, T, F>(
    transaction: &'a T,
    client: Option<&'a mut impl Client<'a>>,
    signers_count: Option<u8>,
) -> Result<XRPAmount<'a>>
where
    T: Transaction<'a, F> + 'static,
    F: IntoEnumIterator + Serialize + core::fmt::Debug + PartialEq,
{
    if let Some(client) = client {
        let net_fee = get_fee(client, None, None).await?;
        let mut base_fee = match transaction.get_transaction_type() {
            TransactionType::EscrowFinish => {
                calculate_base_fee_for_escrow_finish(transaction, net_fee.clone())
            }
            // TODO: same for TransactionType::AMMCreate
            TransactionType::AccountDelete => get_owner_reserve_from_response(client).await?,
            _ => net_fee.clone(),
        };
        if let Some(signers_count) = signers_count {
            base_fee += net_fee.clone() * (1 + signers_count);
        }

        Ok(base_fee.ceil())
    } else {
        let net_fee = XRPAmount::from("10");
        let mut base_fee = match transaction.get_transaction_type() {
            TransactionType::EscrowFinish => {
                calculate_base_fee_for_escrow_finish(transaction, net_fee.clone())
            }
            // TODO: same for TransactionType::AMMCreate
            TransactionType::AccountDelete => XRPAmount::from(OWNER_RESERVE),
            _ => net_fee.clone(),
        };
        if let Some(signers_count) = signers_count {
            base_fee += net_fee.clone() * (1 + signers_count);
        }

        Ok(base_fee.ceil())
    }
}

async fn get_owner_reserve_from_response<'a>(
    client: &'a mut impl Client<'a>,
) -> Result<XRPAmount<'a>> {
    let owner_reserve_response = client
        .request::<ServerStateResult<'a>>(ServerState::new(None))
        .await?;
    match owner_reserve_response.result.state.validated_ledger {
        Some(validated_ledger) => Ok(validated_ledger.reserve_base),
        None => Err!(XRPLModelException::MissingField("validated_ledger")),
    }
}

fn calculate_base_fee_for_escrow_finish<'a, T, F>(
    transaction: &'a T,
    net_fee: XRPAmount<'a>,
) -> XRPAmount<'a>
where
    T: Transaction<'a, F> + 'static, // must outlive 'static for downcasting
    F: IntoEnumIterator + Serialize + core::fmt::Debug + PartialEq,
{
    if transaction.get_transaction_type() == TransactionType::EscrowFinish {
        // cast type to transaction `EscrowFinish`
        let escrow_finish: Option<&EscrowFinish<'a>> =
            (transaction as &dyn Any).downcast_ref::<EscrowFinish>();
        if let Some(escrow_finish) = escrow_finish {
            if let Some(fulfillment) = escrow_finish.fulfillment.clone() {
                return calculate_based_on_fulfillment(fulfillment, net_fee);
            }
        }
    }

    net_fee
}

fn calculate_based_on_fulfillment<'a>(
    fulfillment: Cow<str>,
    net_fee: XRPAmount<'a>,
) -> XRPAmount<'a> {
    let fulfillment_bytes: Vec<u8> = fulfillment.chars().map(|c| c as u8).collect();
    let net_fee_f64: f64 = net_fee.try_into().unwrap();
    let base_fee_string =
        (net_fee_f64 * (33.0 + (fulfillment_bytes.len() as f64 / 16.0))).to_string();
    let base_fee_decimal: Decimal = base_fee_string.parse().unwrap();
    XRPAmount::from(base_fee_decimal.ceil())
}

async fn set_network_id_and_build_version<'a>(client: &'a mut impl Client<'a>) -> Result<()> {
    if client.get_common_fields().is_none() {
        client.set_common_fields().await?;
    }

    Ok(())
}

fn txn_needs_network_id<'a>(client: &'a mut impl Client<'a>) -> bool {
    if let Some(common_fields) = client.get_common_fields() {
        common_fields.network_id.unwrap() > RESTRICTED_NETWORKS as u32
            && is_not_later_rippled_version(
                REQUIRED_NETWORKID_VERSION.into(),
                common_fields.build_version.unwrap().into(),
            )
    } else {
        false
    }
}

fn is_not_later_rippled_version(source: String, target: String) -> bool {
    if source == target {
        true
    } else {
        let source_decomp = source
            .split('.')
            .map(|i| i.to_string())
            .collect::<Vec<String>>();
        let target_decomp = target
            .split('.')
            .map(|i| i.to_string())
            .collect::<Vec<String>>();
        let (source_major, source_minor) = (
            source_decomp[0].parse::<u8>().unwrap(),
            source_decomp[1].parse::<u8>().unwrap(),
        );
        let (target_major, target_minor) = (
            target_decomp[0].parse::<u8>().unwrap(),
            target_decomp[1].parse::<u8>().unwrap(),
        );

        if source_major != target_major {
            source_major < target_major
        } else if source_minor != target_minor {
            source_minor < target_minor
        } else {
            let source_patch = source_decomp[2]
                .split('-')
                .map(|i| i.to_string())
                .collect::<Vec<String>>();
            let target_patch = target_decomp[2]
                .split('-')
                .map(|i| i.to_string())
                .collect::<Vec<String>>();
            let source_patch_version = source_patch[0].parse::<u8>().unwrap();
            let target_patch_version = target_patch[0].parse::<u8>().unwrap();

            if source_patch_version != target_patch_version {
                source_patch_version < target_patch_version
            } else if source_patch.len() != target_patch.len() {
                source_patch.len() < target_patch.len()
            } else if source_patch.len() == 2 {
                if source_patch[1].chars().nth(0).unwrap()
                    != target_patch[1].chars().nth(0).unwrap()
                {
                    source_patch[1] < target_patch[1]
                } else if source_patch[1].starts_with('b') {
                    &source_patch[1][1..] < &target_patch[1][1..]
                } else {
                    &source_patch[1][2..] < &target_patch[1][2..]
                }
            } else {
                false
            }
        }
    }
}
