use alloc::borrow::Cow;
use anyhow::Result;

use crate::{
    core::addresscodec::{is_valid_xaddress, xaddress_to_classic_address},
    models::{ledger::AccountRoot, requests::AccountInfo, results},
    Err,
};

use super::clients::AsyncClient;

pub async fn get_next_valid_seq_number(
    address: Cow<'_, str>,
    client: &impl AsyncClient,
    ledger_index: Option<Cow<'_, str>>,
) -> Result<u32> {
    let account_info =
        get_account_root(address, client, ledger_index.unwrap_or("current".into())).await?;
    Ok(account_info.sequence)
}

pub async fn get_account_root<'a>(
    address: Cow<'_, str>,
    client: &impl AsyncClient,
    ledger_index: Cow<'_, str>,
) -> Result<AccountRoot<'a>> {
    let mut classic_address = address;
    if is_valid_xaddress(&classic_address) {
        classic_address = match xaddress_to_classic_address(&classic_address) {
            Ok(addr) => addr.0.into(),
            Err(e) => return Err!(e),
        };
    }
    let account_info = client
        .request::<results::AccountInfo, _>(AccountInfo::new(
            None,
            classic_address,
            None,
            Some(ledger_index),
            None,
            None,
            None,
        ))
        .await?;

    Ok(account_info.try_into_result()?.account_data)
}
