use core::convert::TryInto;
use core::fmt::Debug;

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
use crate::models::transactions::Transaction;
use crate::models::transactions::TransactionType;
use crate::models::Model;
use crate::Err;

use super::account::get_next_valid_seq_number;
use super::clients::Client;
use super::clients::CommonFields;
use super::ledger::get_fee;
use super::ledger::get_latest_validated_ledger_sequence;

pub mod exceptions;

const OWNER_RESERVE: &'static str = "2000000"; // 2 XRP
const RESTRICTED_NETWORKS: u16 = 1024;
const REQUIRED_NETWORKID_VERSION: &'static str = "1.11.0";
const LEDGER_OFFSET: u8 = 20;

pub async fn autofill<'a, F, T>(
    transaction: &'a mut T,
    client: &'a impl Client<'a>,
    signers_count: Option<u8>,
) -> Result<()>
where
    T: Transaction<'a, F> + Model + Clone + 'static,
    F: IntoEnumIterator + Serialize + Debug + PartialEq + 'a,
{
    let txn = transaction.clone();
    let txn_common_fields = transaction.get_mut_common_fields();
    let common_fields = client.get_common_fields().await?;
    if txn_common_fields.network_id.is_none() && txn_needs_network_id(common_fields.clone()) {
        txn_common_fields.network_id = common_fields.network_id;
    }
    if txn_common_fields.sequence.is_none() {
        txn_common_fields.sequence =
            Some(get_next_valid_seq_number(txn_common_fields.account.clone(), client, None).await?);
    }
    if txn_common_fields.fee.is_none() {
        txn_common_fields.fee =
            Some(calculate_fee_per_transaction_type(txn, Some(client), signers_count).await?);
    }
    if txn_common_fields.last_ledger_sequence.is_none() {
        let ledger_sequence = get_latest_validated_ledger_sequence(client).await?;
        txn_common_fields.last_ledger_sequence = Some(ledger_sequence + LEDGER_OFFSET as u32);
    }

    Ok(())
}

async fn calculate_fee_per_transaction_type<'a, T, F>(
    transaction: T,
    client: Option<&'a impl Client<'a>>,
    signers_count: Option<u8>,
) -> Result<XRPAmount<'a>>
where
    T: Transaction<'a, F> + 'static,
    F: IntoEnumIterator + Serialize + Debug + PartialEq,
{
    let mut net_fee = XRPAmount::from("10");
    let mut base_fee;

    if let Some(client) = client {
        net_fee = get_fee(client, None, None).await?;
        base_fee = match transaction.get_transaction_type() {
            TransactionType::EscrowFinish => calculate_base_fee_for_escrow_finish(
                net_fee.clone(),
                Some(transaction.get_field_value("fulfillment").unwrap().into()),
            ),
            // TODO: same for TransactionType::AMMCreate
            TransactionType::AccountDelete => get_owner_reserve_from_response(client).await?,
            _ => net_fee.clone(),
        };
    } else {
        base_fee = match transaction.get_transaction_type() {
            TransactionType::EscrowFinish => calculate_base_fee_for_escrow_finish(
                net_fee.clone(),
                Some(transaction.get_field_value("fulfillment").unwrap().into()),
            ),
            // TODO: same for TransactionType::AMMCreate
            TransactionType::AccountDelete => XRPAmount::from(OWNER_RESERVE),
            _ => net_fee.clone(),
        };
    }

    if let Some(signers_count) = signers_count {
        base_fee += net_fee * (1 + signers_count);
    }

    Ok(base_fee.ceil())
}

async fn get_owner_reserve_from_response<'a>(client: &'a impl Client<'a>) -> Result<XRPAmount<'a>> {
    let owner_reserve_response = client
        .request::<ServerStateResult<'a>>(ServerState::new(None))
        .await?;
    match owner_reserve_response.result.state.validated_ledger {
        Some(validated_ledger) => Ok(validated_ledger.reserve_base),
        None => Err!(XRPLModelException::MissingField("validated_ledger")),
    }
}

fn calculate_base_fee_for_escrow_finish<'a>(
    net_fee: XRPAmount<'a>,
    fulfillment: Option<Cow<str>>,
) -> XRPAmount<'a> {
    if let Some(fulfillment) = fulfillment {
        return calculate_based_on_fulfillment(fulfillment, net_fee);
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

fn txn_needs_network_id<'a>(common_fields: CommonFields<'a>) -> bool {
    common_fields.network_id.unwrap() > RESTRICTED_NETWORKS as u32
        && is_not_later_rippled_version(
            REQUIRED_NETWORKID_VERSION.into(),
            common_fields.build_version.unwrap().into(),
        )
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
