use core::cmp::max;

use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use strum::IntoEnumIterator;

use crate::models::amount::XRPAmount;
use crate::models::requests::Fee;
use crate::models::transactions::CommonFields;
use crate::models::transactions::EscrowFinish;
use crate::models::transactions::Transaction;
use crate::models::transactions::TransactionType;

use super::clients::Client;
use super::ledger::get_fee;

async fn check_fee<'a, 'de, F, R: Serialize, T: Deserialize<'de>>(
    transaction: &'a mut impl Transaction<F>,
    client: Option<&'a mut impl Client>,
    signers_count: Option<u8>,
) where
    F: IntoEnumIterator + Serialize + core::fmt::Debug + PartialEq,
{
    let expected_fee = max(
        0.1,
        calculate_fee_per_transaction_type(transaction, client, signers_count).await,
    );
}

async fn calculate_fee_per_transaction_type<'a, F>(
    transaction: &'a impl Transaction<F>,
    client: Option<&'a mut impl Client>,
    signers_count: Option<u8>,
) -> Result<f64>
where
    F: IntoEnumIterator + Serialize + core::fmt::Debug + PartialEq,
{
    let net_fee = if let Some(client) = client {
        get_fee(client, None, None).await?
    } else {
        XRPAmount::from("10")
    };

    let base_fee = net_fee.clone();

    if transaction.get_transaction_type() == TransactionType::EscrowFinish {
        // cast type to transaction `EscrowFinish`
        let escrow_finish = transaction.as_any().downcast_ref::<EscrowFinish>().unwrap();
    }

    todo!()
}
