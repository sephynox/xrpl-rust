use alloc::borrow::Cow;
use anyhow::Result;

use crate::{
    core::addresscodec::{is_valid_xaddress, xaddress_to_classic_address},
    models::{ledger::AccountRoot, requests::AccountInfo, results},
};

use super::clients::Client;

pub async fn get_next_valid_seq_number<'a>(
    address: Cow<'a, str>,
    client: &'a impl Client<'a>,
    ledger_index: Option<Cow<'a, str>>,
) -> Result<u32> {
    let account_info =
        get_account_root(address, client, ledger_index.unwrap_or("current".into())).await?;
    Ok(account_info.sequence)
}

pub async fn get_account_root<'a>(
    address: Cow<'a, str>,
    client: &'a impl Client<'a>,
    ledger_index: Cow<'a, str>,
) -> Result<AccountRoot<'a>> {
    let mut classic_address = address;
    if is_valid_xaddress(&classic_address) {
        classic_address = xaddress_to_classic_address(&classic_address)
            .unwrap()
            .0
            .into();
    }
    let account_info = client
        .request::<results::AccountInfo>(AccountInfo::new(
            None,
            classic_address,
            None,
            Some(ledger_index),
            None,
            None,
            None,
        ))
        .await?;

    Ok(account_info.result.account_data)
}
