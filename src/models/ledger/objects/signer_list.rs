use crate::_serde::lgr_obj_flags;
use crate::models::ledger::LedgerEntryType;
use crate::models::{Amount, Currency, Model};
use alloc::borrow::Cow;
use alloc::vec::Vec;
use derive_new::new;
use serde::{ser::SerializeMap, Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum_macros::{AsRefStr, Display, EnumIter};

use crate::serialize_with_tag;
use serde_with::skip_serializing_none;

#[derive(
    Debug, Eq, PartialEq, Clone, Serialize_repr, Deserialize_repr, Display, AsRefStr, EnumIter,
)]
#[repr(u32)]
pub enum SignerListFlag {
    LsfOneOwnerCount = 0x00010000,
}

serialize_with_tag! {
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, new, Default)]
pub struct SignerEntry<'a>{
    pub account: &'a str,
    pub signer_weight: u16,
    pub wallet_locator: Option<&'a str>,
}
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SignerList<'a> {
    ledger_entry_type: LedgerEntryType,
    #[serde(with = "lgr_obj_flags")]
    flags: Vec<SignerListFlag>,
    /// The object ID of a single object to retrieve from the ledger, as a
    /// 64-character (256-bit) hexadecimal string.
    #[serde(rename = "index")]
    pub index: &'a str,
    pub owner_node: &'a str,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: &'a str,
    pub previous_txn_lgr_seq: u32,
    pub signer_entries: Vec<SignerEntry<'a>>,
    #[serde(rename = "SignerListID")]
    pub signer_list_id: u32,
    pub signer_quorum: u32,
}

impl<'a> Default for SignerList<'a> {
    fn default() -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::SignerList,
            flags: Default::default(),
            index: Default::default(),
            owner_node: Default::default(),
            previous_txn_id: Default::default(),
            previous_txn_lgr_seq: Default::default(),
            signer_entries: Default::default(),
            signer_list_id: Default::default(),
            signer_quorum: Default::default(),
        }
    }
}

impl<'a> Model for SignerList<'a> {}

impl<'a> SignerList<'a> {
    pub fn new(
        flags: Vec<SignerListFlag>,
        index: &'a str,
        owner_node: &'a str,
        previous_txn_id: &'a str,
        previous_txn_lgr_seq: u32,
        signer_entries: Vec<SignerEntry<'a>>,
        signer_list_id: u32,
        signer_quorum: u32,
    ) -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::SignerList,
            flags,
            index,
            owner_node,
            previous_txn_id,
            previous_txn_lgr_seq,
            signer_entries,
            signer_list_id,
            signer_quorum,
        }
    }
}

#[cfg(test)]
mod test_serde {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_serialize() {
        let signer_list = SignerList::new(
            vec![],
            "A9C28A28B85CD533217F5C0A0C7767666B093FA58A0F2D80026FCC4CD932DDC7",
            "0000000000000000",
            "5904C0DC72C58A83AEFED2FFC5386356AA83FCA6A88C89D00646E51E687CDBE4",
            16061435,
            vec![
                SignerEntry::new("rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW", 2, None),
                SignerEntry::new("raKEEVSGnKSD9Zyvxu4z6Pqpm4ABH8FS6n", 1, None),
                SignerEntry::new("rUpy3eEg8rqjqfUoLeBnZkscbKbFsKXC3v", 1, None),
            ],
            0,
            3,
        );
        let signer_list_json = serde_json::to_string(&signer_list).unwrap();
        let actual = signer_list_json.as_str();
        let expected = r#"{"LedgerEntryType":"SignerList","Flags":0,"index":"A9C28A28B85CD533217F5C0A0C7767666B093FA58A0F2D80026FCC4CD932DDC7","OwnerNode":"0000000000000000","PreviousTxnID":"5904C0DC72C58A83AEFED2FFC5386356AA83FCA6A88C89D00646E51E687CDBE4","PreviousTxnLgrSeq":16061435,"SignerEntries":[{"SignerEntry":{"Account":"rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW","SignerWeight":2,"WalletLocator":null}},{"SignerEntry":{"Account":"raKEEVSGnKSD9Zyvxu4z6Pqpm4ABH8FS6n","SignerWeight":1,"WalletLocator":null}},{"SignerEntry":{"Account":"rUpy3eEg8rqjqfUoLeBnZkscbKbFsKXC3v","SignerWeight":1,"WalletLocator":null}}],"SignerListID":0,"SignerQuorum":3}"#;

        assert_eq!(expected, actual);
    }
}
