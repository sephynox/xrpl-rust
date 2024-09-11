use alloc::{borrow::Cow, vec::Vec};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;
use strum_macros::{AsRefStr, Display, EnumIter};

use crate::models::{Amount, FlagCollection, Model, XRPAmount};

use super::{CommonFields, Memo, Signer, Transaction, TransactionType, XChainBridge};

#[derive(
    Debug, Eq, PartialEq, Clone, Serialize_repr, Deserialize_repr, Display, AsRefStr, EnumIter,
)]
#[repr(u32)]
pub enum XChainModifyBridgeFlags {
    /// Clears the MinAccountCreateAmount of the bridge.
    TfClearAccountCreateAmount = 0x00010000,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[skip_serializing_none]
pub struct XChainModifyBridge<'a> {
    #[serde(flatten)]
    pub common_fields: CommonFields<'a, XChainModifyBridgeFlags>,
    #[serde(rename = "XChainBridge")]
    pub xchain_bridge: XChainBridge<'a>,
    pub min_account_create_amount: Option<Amount<'a>>,
    pub signature_reward: Option<Amount<'a>>,
}

impl Model for XChainModifyBridge<'_> {}

impl<'a> Transaction<'a, XChainModifyBridgeFlags> for XChainModifyBridge<'a> {
    fn get_common_fields(&self) -> &CommonFields<'_, XChainModifyBridgeFlags> {
        &self.common_fields
    }

    fn get_mut_common_fields(&mut self) -> &mut CommonFields<'a, XChainModifyBridgeFlags> {
        &mut self.common_fields
    }

    fn get_transaction_type(&self) -> super::TransactionType {
        TransactionType::XChainModifyBridge
    }
}

impl<'a> XChainModifyBridge<'a> {
    pub fn new(
        account: Cow<'a, str>,
        account_txn_id: Option<Cow<'a, str>>,
        fee: Option<XRPAmount<'a>>,
        flags: Option<FlagCollection<XChainModifyBridgeFlags>>,
        last_ledger_sequence: Option<u32>,
        memos: Option<Vec<Memo>>,
        sequence: Option<u32>,
        signers: Option<Vec<Signer<'a>>>,
        source_tag: Option<u32>,
        ticket_sequence: Option<u32>,
        xchain_bridge: XChainBridge<'a>,
        min_account_create_amount: Option<Amount<'a>>,
        signature_reward: Option<Amount<'a>>,
    ) -> XChainModifyBridge<'a> {
        XChainModifyBridge {
            common_fields: CommonFields {
                account,
                transaction_type: TransactionType::XChainModifyBridge,
                account_txn_id,
                fee,
                flags: flags.unwrap_or_default(),
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
            xchain_bridge,
            min_account_create_amount,
            signature_reward,
        }
    }
}
