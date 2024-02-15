use core::cmp::min;

use alloc::string::ToString;
use anyhow::Result;
use serde_json::Value;

use crate::models::{amount::XRPAmount, requests::Fee};

use super::clients::Client;

pub enum FeeType {
    Open,
    Minimum,
    Dynamic,
}

pub async fn get_fee<'a>(
    client: &'a mut impl Client,
    max_fee: Option<u16>,
    fee_type: Option<FeeType>,
) -> Result<XRPAmount<'a>> {
    let fee_request = Fee::new(None);
    match client.request(fee_request).await {
        Ok(response) => {
            let response_value = serde_json::to_value(&response).unwrap();
            let drops = response_value.get("result").unwrap().get("drops").unwrap();
            let fee = match_fee_type(fee_type, drops);

            if let Some(max_fee) = max_fee {
                Ok(XRPAmount::from(min(max_fee, fee).to_string()))
            } else {
                Ok(XRPAmount::from(fee.to_string()))
            }
        }
        Err(err) => Err(err),
    }
}

fn match_fee_type(fee_type: Option<FeeType>, drops: &Value) -> u16 {
    match fee_type {
        None | Some(FeeType::Open) => drops
            .get("open_ledger_fee")
            .unwrap()
            .to_string()
            .parse()
            .unwrap(),
        Some(FeeType::Minimum) => drops
            .get("minimum_fee")
            .unwrap()
            .to_string()
            .parse()
            .unwrap(),
        Some(FeeType::Dynamic) => unimplemented!("Dynamic fee calculation not yet implemented"),
    }
}
