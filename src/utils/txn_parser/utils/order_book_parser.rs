use alloc::{dbg, string::ToString, vec::Vec};
use bigdecimal::BigDecimal;

use crate::{
    models::{
        ledger::objects::LedgerEntryType,
        transactions::metadata::{NodeType, TransactionMetadata},
        Amount,
    },
    utils::{drops_to_xrp, exceptions::XRPLUtilsResult},
};

use super::{
    nodes::{normalize_nodes, NormalizedNode},
    parser::group_offers_by_account,
    AccountObjectGroup, AccountOfferChange, AccountOfferChanges, Balance, OfferChange, OfferStatus,
};

const LSF_SELL: u32 = 0x00020000;

enum OfferSide {
    TakerGets,
    TakerPays,
}

fn get_offer_status(node: &NormalizedNode<'_>) -> OfferStatus {
    match node.node_type {
        NodeType::CreatedNode => OfferStatus::Created,
        NodeType::ModifiedNode => OfferStatus::PartiallyFilled,
        NodeType::DeletedNode => {
            if node.previous_fields.is_some() {
                // a filled offer has previous fields
                OfferStatus::Filled
            } else {
                OfferStatus::Cancelled
            }
        }
    }
}

fn calculate_delta(
    previous_balance: &Balance,
    final_balance: &Balance,
) -> XRPLUtilsResult<BigDecimal> {
    let previous_value: BigDecimal = previous_balance.value.parse()?;
    let final_value: BigDecimal = final_balance.value.parse()?;

    Ok(final_value - previous_value)
}

fn derive_currency_amount<'a: 'b, 'b>(currency_amount: Amount<'a>) -> XRPLUtilsResult<Balance<'b>> {
    match currency_amount {
        Amount::XRPAmount(amount) => Ok(Balance {
            currency: "XRP".into(),
            value: drops_to_xrp(amount.0.as_ref())?.into(),
            issuer: None,
        }),
        Amount::IssuedCurrencyAmount(amount) => Ok(Balance {
            currency: amount.currency.clone(),
            value: amount.value.clone(),
            issuer: Some(amount.issuer.clone()),
        }),
    }
}

fn get_change_amount<'a: 'b, 'b>(
    node: NormalizedNode<'a>,
    side: OfferSide,
) -> XRPLUtilsResult<Option<Balance<'b>>> {
    if let Some(new_fields) = &node.new_fields {
        let amount = match side {
            OfferSide::TakerGets => &new_fields.taker_gets,
            OfferSide::TakerPays => &new_fields.taker_pays,
        };
        if let Some(amount) = amount {
            Ok(Some(derive_currency_amount(amount.clone())?))
        } else {
            Ok(None)
        }
    } else if let Some(final_fields) = &node.final_fields {
        let final_fields_amount = match side {
            OfferSide::TakerGets => &final_fields.taker_gets,
            OfferSide::TakerPays => &final_fields.taker_pays,
        };
        let previous_fields_amount = match side {
            OfferSide::TakerGets => &node.previous_fields.as_ref().unwrap().taker_gets,
            OfferSide::TakerPays => &node.previous_fields.as_ref().unwrap().taker_pays,
        };
        if let (Some(final_fields_amount), Some(previous_fields_amount)) =
            (final_fields_amount, previous_fields_amount)
        {
            let final_balance = derive_currency_amount(final_fields_amount.clone())?;
            let previous_balance = derive_currency_amount(previous_fields_amount.clone())?;
            let change = calculate_delta(&previous_balance, &final_balance)?;
            Ok(Some(Balance {
                currency: final_balance.currency,
                value: change.to_string().into(),
                issuer: final_balance.issuer,
            }))
        } else if let (Some(final_fields_amount), None) =
            (final_fields_amount, previous_fields_amount)
        {
            let final_balance = derive_currency_amount(final_fields_amount.clone())?;
            let final_balance_value: BigDecimal = final_balance.value.parse()?;
            let value: BigDecimal = 0 - final_balance_value;
            Ok(Some(Balance {
                currency: final_balance.currency,
                value: value.to_string().into(),
                issuer: final_balance.issuer,
            }))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

fn get_quality(taker_gets: &Balance, taker_pays: &Balance) -> XRPLUtilsResult<BigDecimal> {
    let taker_gets_value: BigDecimal = taker_gets.value.parse()?;
    let taker_pays_value: BigDecimal = taker_pays.value.parse()?;
    let quality = taker_pays_value / taker_gets_value;

    Ok(quality.normalized())
}

fn get_offer_change<'a: 'b, 'b>(
    node: NormalizedNode<'a>,
) -> XRPLUtilsResult<Option<AccountOfferChange<'b>>> {
    let status = get_offer_status(&node);
    let taker_gets = get_change_amount(node.clone(), OfferSide::TakerGets)?;
    let taker_pays = get_change_amount(node.clone(), OfferSide::TakerPays)?;
    let account = if let Some(new_fields) = &node.new_fields {
        new_fields.account.as_ref().map(|account| account)
    } else if let Some(final_fields) = &node.final_fields {
        final_fields.account.as_ref().map(|account| account)
    } else {
        None
    };
    let sequence = if let Some(new_fields) = &node.new_fields {
        new_fields.sequence
    } else if let Some(final_fields) = &node.final_fields {
        final_fields.sequence
    } else {
        0
    };
    let flags = if let Some(new_fields) = &node.new_fields {
        new_fields.flags
    } else if let Some(final_fields) = &node.final_fields {
        final_fields.flags
    } else {
        0
    };
    if taker_gets.is_none() || taker_pays.is_none() || account.is_none() {
        dbg!("required missing");
        return Ok(None);
    }
    let taker_gets = taker_gets.unwrap();
    let taker_pays = taker_pays.unwrap();
    let account = account.unwrap();

    let expiration_time = if let Some(new_fields) = &node.new_fields {
        new_fields.expiration
    } else if let Some(final_fields) = &node.final_fields {
        final_fields.expiration
    } else {
        None
    };
    let quality = get_quality(&taker_gets, &taker_pays)?;
    let offer_change = OfferChange {
        flags: flags.try_into()?,
        taker_gets: taker_gets.into(),
        taker_pays: taker_pays.into(),
        sequence,
        status,
        maker_exchange_rate: Some(quality),
        expiration_time,
    };

    Ok(Some(AccountOfferChange {
        maker_account: account.clone(),
        offer_change,
    }))
}

fn group_offer_changes_by_account<'a: 'b, 'b>(
    account_offer_changes: Vec<AccountOfferChange<'a>>,
) -> XRPLUtilsResult<Vec<AccountOfferChanges<'b>>> {
    let grouped_offer_changes = group_offers_by_account(account_offer_changes);
    let mut result = Vec::new();
    for AccountObjectGroup {
        account,
        account_offer_changes,
        ..
    } in grouped_offer_changes
    {
        let offer_changes = account_offer_changes
            .iter()
            .map(|change| change.offer_change.clone())
            .collect::<Vec<_>>();
        result.push(AccountOfferChanges {
            account,
            offer_changes,
        })
    }
    Ok(result)
}

pub fn compute_order_book_changes<'a: 'b, 'b>(
    metadata: &'a TransactionMetadata<'a>,
) -> XRPLUtilsResult<Vec<AccountOfferChanges<'b>>> {
    let normalized_nodes = normalize_nodes(metadata);
    let offer_nodes = normalized_nodes
        .iter()
        .filter(|node| node.ledger_entry_type == LedgerEntryType::Offer)
        .cloned()
        .collect::<Vec<_>>();
    let offer_changes = offer_nodes
        .into_iter()
        .filter_map(|node| get_offer_change(node).ok().flatten())
        .collect::<Vec<_>>();

    group_offer_changes_by_account(offer_changes)
}
