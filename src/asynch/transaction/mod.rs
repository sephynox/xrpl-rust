use core::any::Any;
use core::convert::TryFrom;
use core::convert::TryInto;

use alloc::borrow::Cow;
use alloc::string::ToString;
use alloc::vec::Vec;
use anyhow::Ok;
use anyhow::Result;
use rust_decimal::Decimal;
use serde::Serialize;
use strum::IntoEnumIterator;

use crate::models::amount::XRPAmount;
use crate::models::requests::ServerState;
use crate::models::transactions::EscrowFinish;
use crate::models::transactions::Transaction;
use crate::models::transactions::TransactionType;
use crate::models::Model;
use crate::Err;

use self::exceptions::XRPLTransactionException;

use super::clients::Client;
use super::clients::XRPLResponse;
use super::ledger::get_fee;

pub mod exceptions;

const OWNER_RESERVE: &'static str = "2000000"; // 2 XRP

async fn check_fee<'a, F, T>(
    transaction: &'a T,
    client: Option<&'a mut impl Client>,
    signers_count: Option<u8>,
) -> Result<Option<&'a mut impl Client>>
where
    T: Transaction<'a, F> + Model + 'static,
    F: IntoEnumIterator + Serialize + core::fmt::Debug + PartialEq + 'a,
{
    let fee_to_high = XRPAmount::from("1000");
    let (calculated_fee, client_ref) =
        calculate_fee_per_transaction_type(transaction, client, signers_count)
            .await
            .unwrap();
    let expected_fee = fee_to_high.max(calculated_fee);
    let common_fields = transaction.as_common_fields();
    if let Some(fee) = &common_fields.fee {
        if fee < &expected_fee {
            return Err!(XRPLTransactionException::FeeUnusuallyHigh(fee.clone()));
        }
    }

    Ok(client_ref)
}

async fn calculate_fee_per_transaction_type<'a, T, F>(
    transaction: &'a T,
    client: Option<&'a mut impl Client>,
    signers_count: Option<u8>,
) -> Result<(XRPAmount<'a>, Option<&'a mut impl Client>)>
where
    T: Transaction<'a, F> + 'static,
    F: IntoEnumIterator + Serialize + core::fmt::Debug + PartialEq,
{
    if let Some(client) = client {
        let (net_fee, client_ref) = get_fee(client, None, None).await?;
        let (mut base_fee, client_) = match transaction.get_transaction_type() {
            TransactionType::EscrowFinish => {
                let base_fee = calculate_base_fee_for_escrow_finish(transaction, net_fee.clone());
                (base_fee, client_ref)
            }
            // TODO: same for TransactionType::AMMCreate
            TransactionType::AccountDelete => {
                let owner_reserve_response = client_ref.request(ServerState::new(None)).await?;
                let owner_reserve = get_owner_reserve_from_response(owner_reserve_response);

                (owner_reserve, client_ref)
            }
            _ => (net_fee.clone(), client_ref),
        };
        if let Some(signers_count) = signers_count {
            base_fee += net_fee.clone() * (1 + signers_count);
        }

        return Ok((base_fee.ceil(), Some(client_)));
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

        return Ok((base_fee.ceil(), None));
    }
}

fn get_owner_reserve_from_response<'a>(response: XRPLResponse) -> XRPAmount<'a> {
    let owner_reserve = response
        .result
        .unwrap()
        .get("state")
        .unwrap()
        .get("validated_ledger")
        .unwrap()
        .get("reserve_inc")
        .unwrap()
        .clone();
    XRPAmount::try_from(owner_reserve).unwrap()
}

fn calculate_base_fee_for_escrow_finish<'a, T, F>(
    transaction: &'a T,
    net_fee: XRPAmount<'a>,
) -> XRPAmount<'a>
where
    T: Transaction<'a, F> + 'static,
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
