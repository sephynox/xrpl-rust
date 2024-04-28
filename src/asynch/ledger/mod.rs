use core::cmp::min;

use alloc::string::ToString;
use anyhow::Result;

use crate::models::{
    amount::XRPAmount,
    requests::{Fee, Ledger},
    results::{
        self,
        fee::{Drops, Fee as FeeResult},
    },
};

use super::clients::Client;

pub async fn get_latest_validated_ledger_sequence<'a>(client: &'a impl Client<'a>) -> Result<u32> {
    let ledger_response = client
        .request::<results::Ledger>(Ledger::new(
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

    Ok(ledger_response.result.ledger_index)
}

pub enum FeeType {
    Open,
    Minimum,
    Dynamic,
}

pub async fn get_fee<'a>(
    client: &'a impl Client<'a>,
    max_fee: Option<u16>,
    fee_type: Option<FeeType>,
) -> Result<XRPAmount<'a>> {
    let fee_request = Fee::new(None);
    match client.request::<FeeResult<'a>>(fee_request).await {
        Ok(response) => {
            let drops = response.result.drops;
            let fee = match_fee_type(fee_type, drops).unwrap();

            if let Some(max_fee) = max_fee {
                Ok(XRPAmount::from(min(max_fee, fee).to_string()))
            } else {
                Ok(XRPAmount::from(fee.to_string()))
            }
        }
        Err(err) => Err(err),
    }
}

fn match_fee_type<'a>(fee_type: Option<FeeType>, drops: Drops<'a>) -> Result<u16> {
    match fee_type {
        None | Some(FeeType::Open) => Ok(drops.open_ledger_fee.0.to_string().parse().unwrap()),
        Some(FeeType::Minimum) => Ok(drops.minimum_fee.0.to_string().parse().unwrap()),
        Some(FeeType::Dynamic) => unimplemented!("Dynamic fee calculation not yet implemented"),
    }
}
