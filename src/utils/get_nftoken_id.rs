use alloc::{borrow::Cow, vec::Vec};

use crate::models::ledger::objects::LedgerEntryType;
use crate::models::transactions::metadata::TransactionMetadata;
use crate::models::transactions::metadata::{AffectedNode, NFTokenMetadata};

/// Flattens objects into a single list, and applies func to every object in objects.
fn flatmap<T, R>(func: impl Fn(T) -> Vec<R>, list_of_items: Vec<T>) -> Vec<R> {
    let mut modified_items: Vec<R> = Vec::new();
    for item in list_of_items {
        modified_items.extend(func(item));
    }
    modified_items
}

pub fn get_nftoken_ids_from_nftokens(nftokens: Vec<NFTokenMetadata<'_>>) -> Vec<Cow<'_, str>> {
    nftokens
        .into_iter()
        .map(|token| token.nftoken.nftoken_id)
        .collect()
}

fn has_nftoken_page(node: &AffectedNode) -> bool {
    match node {
        AffectedNode::CreatedNode {
            ledger_entry_type, ..
        } => ledger_entry_type == &LedgerEntryType::NFTokenPage,
        AffectedNode::ModifiedNode {
            ledger_entry_type,
            previous_fields,
            ..
        } => {
            ledger_entry_type == &LedgerEntryType::NFTokenPage
                && previous_fields.is_some()
                && previous_fields.as_ref().unwrap().nftokens.is_some()
        }
        _ => false,
    }
}

fn get_previous_nftokens<'a>(node: &'a AffectedNode) -> Vec<NFTokenMetadata<'a>> {
    match node {
        AffectedNode::ModifiedNode {
            previous_fields, ..
        } => {
            if let Some(previous_fields) = previous_fields {
                if let Some(new_nftokens) = &previous_fields.nftokens {
                    new_nftokens.clone()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        }
        _ => Vec::new(),
    }
}

fn get_new_nftokens<'a>(node: &'a AffectedNode) -> Vec<NFTokenMetadata<'a>> {
    match node {
        AffectedNode::ModifiedNode { final_fields, .. } => {
            if let Some(final_fields) = final_fields {
                if let Some(new_nftokens) = &final_fields.nftokens {
                    new_nftokens.clone()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        }
        AffectedNode::CreatedNode { new_fields, .. } => {
            if let Some(new_nftokens) = &new_fields.nftokens {
                new_nftokens.clone()
            } else {
                Vec::new()
            }
        }
        _ => Vec::new(),
    }
}

pub fn get_nftoken_id<'a: 'b, 'b>(meta: &'a TransactionMetadata<'a>) -> Option<Cow<'b, str>> {
    let affected_nodes: Vec<&AffectedNode> = meta
        .affected_nodes
        .iter()
        .filter(|node| has_nftoken_page(node))
        .collect();

    if affected_nodes.is_empty() {
        return None;
    }

    let previous_token_ids: Vec<Cow<'_, str>> =
        get_nftoken_ids_from_nftokens(flatmap(get_previous_nftokens, affected_nodes.clone()));

    let final_token_ids: Vec<Cow<'_, str>> =
        get_nftoken_ids_from_nftokens(flatmap(get_new_nftokens, affected_nodes));

    final_token_ids
        .into_iter()
        .find(|id| !previous_token_ids.contains(id))
}

#[cfg(test)]
mod test {
    use lazy_static::lazy_static;
    use serde::{Deserialize, Serialize};

    use crate::models::transactions::metadata::TransactionMetadata;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Txn {
        pub meta: TransactionMetadata<'static>,
    }

    fn load_tests() -> &'static Option<(Txn, Txn)> {
        pub const NFTOKEN_MINT_RESPONSE_1: &str =
            include_str!("./test_data/transaction_jsons/nftokenmint_response1.json");
        pub const NFTOKEN_MINT_RESPONSE_2: &str =
            include_str!("./test_data/transaction_jsons/nftokenmint_response2.json");

        lazy_static! {
            static ref TEST_CASES: Option<(Txn, Txn,)> = Some((
                serde_json::from_str(NFTOKEN_MINT_RESPONSE_1).expect("load_tests_1"),
                serde_json::from_str(NFTOKEN_MINT_RESPONSE_2).expect("load_tests_2"),
            ));
        }

        &TEST_CASES
    }

    #[test]
    fn test_nftoken_id() {
        let (txn1, txn2) = load_tests()
            .as_ref()
            .expect("test_decoding_a_valid_nftoken_id");
        let nftoken_id1 = super::get_nftoken_id(&txn1.meta);
        let nftoken_id2 = super::get_nftoken_id(&txn2.meta);
        let expected_nftoken_id1 =
            Some("00081388DC1AB4937C899037B2FDFC3CB20F6F64E73120BB5F8AA66A00000228".into());
        let expected_nftoken_id2 =
            Some("0008125CBE4B401B2F62ED35CC67362165AA813CCA06316FFA766254000003EE".into());

        assert_eq!(nftoken_id1, expected_nftoken_id1);
        assert_eq!(nftoken_id2, expected_nftoken_id2);
    }
}
