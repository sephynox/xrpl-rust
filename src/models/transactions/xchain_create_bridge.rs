use alloc::{borrow::Cow, vec::Vec};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{Amount, FlagCollection, Model, NoFlags, XRPAmount};

use super::{CommonFields, Memo, Signer, Transaction, TransactionType, XChainBridge};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[skip_serializing_none]
pub struct XChainCreateBridge<'a> {
    #[serde(flatten)]
    pub common_fields: CommonFields<'a, NoFlags>,
    pub signature_reward: Amount<'a>,
    #[serde(rename = "XChainBridge")]
    pub xchain_bridge: XChainBridge<'a>,
    pub min_account_create_amount: Option<Amount<'a>>,
}

impl Model for XChainCreateBridge<'_> {}

impl<'a> Transaction<'a, NoFlags> for XChainCreateBridge<'a> {
    fn get_transaction_type(&self) -> super::TransactionType {
        TransactionType::XChainCreateBridge
    }

    fn get_common_fields(&self) -> &CommonFields<'_, NoFlags> {
        &self.common_fields
    }

    fn get_mut_common_fields(&mut self) -> &mut CommonFields<'a, NoFlags> {
        &mut self.common_fields
    }
}

impl<'a> XChainCreateBridge<'a> {
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
        signature_reward: Amount<'a>,
        xchain_bridge: XChainBridge<'a>,
        min_account_create_amount: Option<Amount<'a>>,
    ) -> XChainCreateBridge<'a> {
        XChainCreateBridge {
            common_fields: CommonFields {
                account,
                transaction_type: TransactionType::XChainCreateBridge,
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
            signature_reward,
            xchain_bridge,
            min_account_create_amount,
        }
    }
}
