use alloc::{borrow::Cow, vec::Vec};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{FlagCollection, Model, NoFlags, XRPAmount};

use super::{CommonFields, Memo, Signer, Transaction, TransactionType, XChainBridge};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[skip_serializing_none]
pub struct XChainCreateClaimID<'a> {
    #[serde(flatten)]
    pub common_fields: CommonFields<'a, NoFlags>,
    pub other_chain_source: Cow<'a, str>,
    pub signature_reward: Cow<'a, str>,
    #[serde(rename = "XChainBridge")]
    pub xchain_bridge: XChainBridge<'a>,
}

impl Model for XChainCreateClaimID<'_> {}

impl<'a> Transaction<'a, NoFlags> for XChainCreateClaimID<'a> {
    fn get_transaction_type(&self) -> super::TransactionType {
        TransactionType::XChainCreateClaimID
    }

    fn get_common_fields(&self) -> &CommonFields<'_, NoFlags> {
        &self.common_fields
    }

    fn get_mut_common_fields(&mut self) -> &mut CommonFields<'a, NoFlags> {
        &mut self.common_fields
    }
}

impl<'a> XChainCreateClaimID<'a> {
    pub fn new(
        account: Cow<'a, str>,
        account_txn_id: Option<Cow<'a, str>>,
        fee: Option<XRPAmount<'a>>,
        last_ledger_sequence: Option<u32>,
        memos: Option<Vec<Memo>>,
        sequence: Option<u32>,
        signers: Option<Vec<Signer<'a>>>,
        source_tag: Option<u32>,
        ticket_sequence: Option<u32>,
        other_chain_source: Cow<'a, str>,
        signature_reward: Cow<'a, str>,
        xchain_bridge: XChainBridge<'a>,
    ) -> XChainCreateClaimID<'a> {
        XChainCreateClaimID {
            common_fields: CommonFields {
                account,
                transaction_type: TransactionType::XChainCreateClaimID,
                account_txn_id,
                fee,
                flags: FlagCollection::default(),
                last_ledger_sequence,
                memos,
                sequence,
                signers,
                source_tag,
                ticket_sequence,
                network_id: None,
                signing_pub_key: None,
                txn_signature: None,
            },
            other_chain_source,
            signature_reward,
            xchain_bridge,
        }
    }
}
