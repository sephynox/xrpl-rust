use core::{cmp::min, convert::TryInto};

use alloc::string::ToString;
use anyhow::Result;

use crate::models::{
    amount::XRPAmount,
    requests::{Fee, Ledger},
    results::{
        self, {Drops, Fee as FeeResult},
    },
};

use super::clients::AsyncClient;

pub async fn get_latest_validated_ledger_sequence(client: &impl AsyncClient) -> Result<u32> {
    let ledger_response = client
        .request::<results::Ledger, _>(Ledger::new(
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
        ))
        .await?;

    Ok(ledger_response.try_into_result()?.ledger_index)
}

pub enum FeeType {
    Open,
    Minimum,
    Dynamic,
}

pub async fn get_fee(
    client: &impl AsyncClient,
    max_fee: Option<u32>,
    fee_type: Option<FeeType>,
) -> Result<XRPAmount<'_>> {
    let fee_request = Fee::new(None);
    match client.request::<FeeResult<'_>, _>(fee_request).await {
        Ok(response) => {
            let drops = response.try_into_result()?.drops;
            let fee = match_fee_type(fee_type, drops)?;

            if let Some(max_fee) = max_fee {
                Ok(XRPAmount::from(min(max_fee, fee).to_string()))
            } else {
                Ok(XRPAmount::from(fee.to_string()))
            }
        }
        Err(err) => Err(err),
    }
}

fn match_fee_type(fee_type: Option<FeeType>, drops: Drops<'_>) -> Result<u32> {
    match fee_type {
        None | Some(FeeType::Open) => Ok(drops.open_ledger_fee.try_into()?),
        Some(FeeType::Minimum) => Ok(drops.minimum_fee.try_into()?),
        Some(FeeType::Dynamic) => unimplemented!("Dynamic fee calculation not yet implemented"),
    }
}
