use alloc::vec::Vec;
use bigdecimal::BigDecimal;

use crate::{
    models::transactions::metadata::TransactionMetadata,
    utils::{exceptions::XRPLUtilsResult, txn_parser::utils::parser::get_value},
};

use super::utils::{
    balance_parser::derive_account_balances, nodes::NormalizedNode, AccountBalances,
};

/// Parses the balance changes of all accounts affected by a transaction from the transaction metadata.
pub fn get_balance_changes<'a: 'b, 'b>(
    meta: &'a TransactionMetadata<'a>,
) -> XRPLUtilsResult<Vec<AccountBalances<'b>>> {
    derive_account_balances(meta, compute_balance_change)
}

/// Get the balance change from a node.
fn compute_balance_change(node: &NormalizedNode) -> XRPLUtilsResult<Option<BigDecimal>> {
    let new_fields = node.new_fields.as_ref();
    let previous_fields = node.previous_fields.as_ref();
    let final_fields = node.final_fields.as_ref();

    if let Some(new_fields) = new_fields {
        if let Some(balance) = &new_fields.balance {
            Ok(Some(get_value(&balance.clone().into())?))
        } else {
            Ok(None)
        }
    } else if let (Some(previous_fields), Some(final_fields)) = (previous_fields, final_fields) {
        if let (Some(prev_balance), Some(final_balance)) =
            (&previous_fields.balance, &final_fields.balance)
        {
            let prev_value = get_value(&prev_balance.clone().into())?;
            let final_value = get_value(&final_balance.clone().into())?;

            Ok(Some(final_value - prev_value))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use core::cell::LazyCell;

    use serde_json::Value;

    use super::*;
    use crate::{
        models::transactions::metadata::TransactionMetadata, utils::txn_parser::utils::Balance,
    };

    #[test]
    fn test_parse_balance_changes() {
        let txn: LazyCell<TransactionMetadata> = LazyCell::new(|| {
            let txn_value: Value =
                serde_json::from_str(include_str!("./test_data/payment_iou.json")).unwrap();
            let txn_meta = txn_value["meta"].clone();
            let txn_meta: TransactionMetadata = serde_json::from_value(txn_meta).unwrap();

            txn_meta
        });
        let expected_balances = Vec::from([
            AccountBalances {
                account: "rKmBGxocj9Abgy25J51Mk1iqFzW9aVF9Tc".into(),
                balances: Vec::from([
                    Balance {
                        currency: "USD".into(),
                        value: "-0.01".into(),
                        issuer: Some("rMwjYedjc7qqtKYVLiAccJSmCwih4LnE2q".into()),
                    },
                    Balance {
                        currency: "XRP".into(),
                        value: "-0.012".into(),
                        issuer: None,
                    },
                ]),
            },
            AccountBalances {
                account: "rMwjYedjc7qqtKYVLiAccJSmCwih4LnE2q".into(),
                balances: Vec::from([
                    Balance {
                        currency: "USD".into(),
                        value: "0.01".into(),
                        issuer: Some("rKmBGxocj9Abgy25J51Mk1iqFzW9aVF9Tc".into()),
                    },
                    Balance {
                        currency: "USD".into(),
                        value: "-0.01".into(),
                        issuer: Some("rLDYrujdKUfVx28T9vRDAbyJ7G2WVXKo4K".into()),
                    },
                ]),
            },
            AccountBalances {
                account: "rLDYrujdKUfVx28T9vRDAbyJ7G2WVXKo4K".into(),
                balances: Vec::from([Balance {
                    currency: "USD".into(),
                    value: "0.01".into(),
                    issuer: Some("rMwjYedjc7qqtKYVLiAccJSmCwih4LnE2q".into()),
                }]),
            },
        ]);
        let balance_changes = get_balance_changes(&txn).unwrap();
        assert_eq!(expected_balances, balance_changes);
    }
}
