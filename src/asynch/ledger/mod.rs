use core::{cmp::min, convert::TryInto};

use alloc::string::ToString;

use crate::models::{
    requests::{fee::Fee, ledger::Ledger},
    results::{self},
    XRPAmount,
};

use super::{clients::XRPLAsyncClient, exceptions::XRPLHelperResult};

pub async fn get_latest_validated_ledger_sequence(
    client: &impl XRPLAsyncClient,
) -> XRPLHelperResult<u32> {
    let ledger_response = client
        .request(
            Ledger::new(
                None,
                None,
                None,
                None,
                None,
                None,
                Some("validated".into()),
                None,
                None,
                None,
            )
            .into(),
        )
        .await?;
    let ledger_result: results::ledger::Ledger = ledger_response.try_into()?;

    Ok(ledger_result.ledger_index)
}

pub async fn get_latest_open_ledger_sequence(
    client: &impl XRPLAsyncClient,
) -> XRPLHelperResult<u32> {
    let ledger_response = client
        .request(
            Ledger::new(
                None,
                None,
                None,
                None,
                None,
                None,
                Some("open".into()),
                None,
                None,
                None,
            )
            .into(),
        )
        .await?;
    let ledger_result: results::ledger::Ledger = ledger_response.try_into()?;

    Ok(ledger_result.ledger_index)
}

pub enum FeeType {
    Open,
    Minimum,
    Dynamic,
}

pub async fn get_fee(
    client: &impl XRPLAsyncClient,
    max_fee: Option<u32>,
    fee_type: Option<FeeType>,
) -> XRPLHelperResult<XRPAmount<'_>> {
    let fee_request = Fee::new(None);
    let response = client.request(fee_request.into()).await?;
    let result: results::fee::Fee = response.try_into()?;
    let drops = result.drops;
    let fee = match_fee_type(fee_type, drops)?;

    if let Some(max_fee) = max_fee {
        Ok(XRPAmount::from(min(max_fee, fee).to_string()))
    } else {
        Ok(XRPAmount::from(fee.to_string()))
    }
}

fn match_fee_type(
    fee_type: Option<FeeType>,
    drops: results::fee::Drops<'_>,
) -> XRPLHelperResult<u32> {
    match fee_type {
        None | Some(FeeType::Open) => Ok(drops.open_ledger_fee.try_into()?),
        Some(FeeType::Minimum) => Ok(drops.minimum_fee.try_into()?),
        Some(FeeType::Dynamic) => unimplemented!("Dynamic fee calculation not yet implemented"),
    }
}
