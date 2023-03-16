use crate::models::ledger::LedgerEntryType;
use crate::models::Model;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DirectoryNode<'a> {
    /// The value 0x0064, mapped to the string `DirectoryNode`, indicates that this object is part
    /// of a Directory.
    pub ledger_entry_type: LedgerEntryType,
    /// A bit-map of boolean flags enabled for this object. Currently, the protocol defines no flags
    /// for `DirectoryNode` objects. The value is always 0.
    pub flags: u32,
    /// The object ID of a single object to retrieve from the ledger, as a
    /// 64-character (256-bit) hexadecimal string.
    #[serde(rename = "index")]
    pub index: &'a str,
    /// (`Offer` Directories only) DEPRECATED. Do not use.
    pub exchange_rate: Option<&'a str>,
    /// The contents of this `Directory`: an array of IDs of other objects.
    pub indexes: Vec<&'a str>,
    /// If this `Directory` consists of multiple pages, this ID links to the next object in the chain,
    /// wrapping around at the end.
    pub index_next: Option<u64>,
    /// If this `Directory` consists of multiple pages, this ID links to the previous object in the
    /// chain, wrapping around at the beginning.
    pub index_previous: Option<u64>,
    /// (Owner Directories only) The address of the account that owns the objects in this directory.
    pub owner: Option<&'a str>,
    /// The ID of root object for this directory.
    pub root_index: &'a str,
    /// (`Offer` `Directories` only) The currency code of the `TakerGets` amount from the offers in this
    /// directory.
    pub taker_gets_currency: Option<&'a str>,
    /// (`Offer` `Directories` only) The issuer of the `TakerGets` amount from the offers in this
    /// directory.
    pub taker_gets_issuer: Option<&'a str>,
    /// (`Offer` `Directories` only) The currency code of the `TakerPays` amount from the offers in this
    /// directory.
    pub taker_pays_currency: Option<&'a str>,
    /// (`Offer` `Directories` only) The issuer of the `TakerPays` amount from the offers in this
    /// directory.
    pub taker_pays_issuer: Option<&'a str>,
}

impl<'a> Default for DirectoryNode<'a> {
    fn default() -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::DirectoryNode,
            flags: Default::default(),
            index: Default::default(),
            exchange_rate: Default::default(),
            indexes: Default::default(),
            index_next: Default::default(),
            index_previous: Default::default(),
            owner: Default::default(),
            root_index: Default::default(),
            taker_gets_currency: Default::default(),
            taker_gets_issuer: Default::default(),
            taker_pays_currency: Default::default(),
            taker_pays_issuer: Default::default(),
        }
    }
}

impl<'a> Model for DirectoryNode<'a> {}

impl<'a> DirectoryNode<'a> {
    pub fn new(
        flags: u32,
        index: &'a str,
        indexes: Vec<&'a str>,
        root_index: &'a str,
        exchange_rate: Option<&'a str>,
        index_next: Option<u64>,
        index_previous: Option<u64>,
        owner: Option<&'a str>,
        taker_gets_currency: Option<&'a str>,
        taker_gets_issuer: Option<&'a str>,
        taker_pays_currency: Option<&'a str>,
        taker_pays_issuer: Option<&'a str>,
    ) -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::DirectoryNode,
            flags,
            index,
            exchange_rate,
            indexes,
            index_next,
            index_previous,
            owner,
            root_index,
            taker_gets_currency,
            taker_gets_issuer,
            taker_pays_currency,
            taker_pays_issuer,
        }
    }
}

#[cfg(test)]
mod test_serde {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_serialize() {
        let directory_node = DirectoryNode::new(
            0,
            "1BBEF97EDE88D40CEE2ADE6FEF121166AFE80D99EBADB01A4F069BA8FF484000",
            vec!["AD7EAE148287EF12D213A251015F86E6D4BD34B3C4A0A1ED9A17198373F908AD"],
            "1BBEF97EDE88D40CEE2ADE6FEF121166AFE80D99EBADB01A4F069BA8FF484000",
            Some("4F069BA8FF484000"),
            None,
            None,
            None,
            Some("0000000000000000000000000000000000000000"),
            Some("0000000000000000000000000000000000000000"),
            Some("0000000000000000000000004A50590000000000"),
            Some("5BBC0F22F61D9224A110650CFE21CC0C4BE13098"),
        );
        let directory_node_json = serde_json::to_string(&directory_node).unwrap();
        let actual = directory_node_json.as_str();
        let expected = r#"{"LedgerEntryType":"DirectoryNode","Flags":0,"index":"1BBEF97EDE88D40CEE2ADE6FEF121166AFE80D99EBADB01A4F069BA8FF484000","ExchangeRate":"4F069BA8FF484000","Indexes":["AD7EAE148287EF12D213A251015F86E6D4BD34B3C4A0A1ED9A17198373F908AD"],"RootIndex":"1BBEF97EDE88D40CEE2ADE6FEF121166AFE80D99EBADB01A4F069BA8FF484000","TakerGetsCurrency":"0000000000000000000000000000000000000000","TakerGetsIssuer":"0000000000000000000000000000000000000000","TakerPaysCurrency":"0000000000000000000000004A50590000000000","TakerPaysIssuer":"5BBC0F22F61D9224A110650CFE21CC0C4BE13098"}"#;

        assert_eq!(expected, actual);
    }
}
