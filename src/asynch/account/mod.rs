use alloc::borrow::Cow;

use crate::{
    core::addresscodec::{is_valid_xaddress, xaddress_to_classic_address},
    models::{
        ledger::objects::AccountRoot,
        requests::{account_info::AccountInfo, account_tx::AccountTx},
        results::{self},
        XRPAmount,
    },
};

use super::{clients::XRPLAsyncClient, exceptions::XRPLHelperResult};

pub async fn does_account_exist<C>(
    address: Cow<'_, str>,
    client: &C,
    ledger_index: Option<Cow<'_, str>>,
) -> XRPLHelperResult<bool>
where
    C: XRPLAsyncClient,
{
    match get_account_root(address, client, ledger_index.unwrap_or("validated".into())).await {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

pub async fn get_next_valid_seq_number(
    address: Cow<'_, str>,
    client: &impl XRPLAsyncClient,
    ledger_index: Option<Cow<'_, str>>,
) -> XRPLHelperResult<u32> {
    let account_info =
        get_account_root(address, client, ledger_index.unwrap_or("current".into())).await?;
    Ok(account_info.sequence)
}

pub async fn get_xrp_balance<'a: 'b, 'b, C>(
    address: Cow<'a, str>,
    client: &C,
    ledger_index: Option<Cow<'a, str>>,
) -> XRPLHelperResult<XRPAmount<'b>>
where
    C: XRPLAsyncClient,
{
    let account_info =
        get_account_root(address, client, ledger_index.unwrap_or("validated".into())).await?;
    match account_info.balance {
        Some(balance) => Ok(balance),
        None => Ok(0.into()),
    }
}

pub async fn get_account_root<'a: 'b, 'b, C>(
    address: Cow<'a, str>,
    client: &C,
    ledger_index: Cow<'a, str>,
) -> XRPLHelperResult<AccountRoot<'b>>
where
    C: XRPLAsyncClient,
{
    let mut classic_address = address;
    if is_valid_xaddress(&classic_address) {
        classic_address = xaddress_to_classic_address(&classic_address)?.0.into();
    }
    let request = AccountInfo::new(
        None,
        classic_address,
        None,
        Some(ledger_index.into()),
        None,
        None,
        None,
    )
    .into();
    let account_info = client.request(request).await?;

    Ok(account_info
        .try_into_result::<results::account_info::AccountInfo<'_>>()?
        .account_data)
}

pub async fn get_latest_transaction<'a: 'b, 'b, C>(
    mut address: Cow<'a, str>,
    client: &C,
) -> XRPLHelperResult<crate::models::results::account_tx::AccountTx<'b>>
where
    C: XRPLAsyncClient,
{
    if is_valid_xaddress(&address) {
        address = xaddress_to_classic_address(&address)?.0.into();
    }
    let account_tx = AccountTx::new(
        None,
        address,
        None,
        Some("validated".into()),
        None,
        None,
        None,
        None,
        Some(1),
        None,
    );
    let response = client.request(account_tx.into()).await?;

    Ok(response.try_into_result::<results::account_tx::AccountTx<'_>>()?)
}
