use alloc::{borrow::Cow, vec::Vec};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::ledger::objects::LedgerEntryType;
use crate::models::requests::LedgerIndex;
use crate::models::{Amount, IssuedCurrencyAmount};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct NFTokenMetadata<'a> {
    #[serde(rename = "NFToken")]
    pub nftoken: NFTokenMetadataFields<'a>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct NFTokenMetadataFields<'a> {
    #[serde(rename = "NFTokenID")]
    pub nftoken_id: Cow<'a, str>,
    #[serde(rename = "URI")]
    pub uri: Cow<'a, str>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Fields<'a> {
    pub account: Option<Cow<'a, str>>,
    pub balance: Option<Amount<'a>>,
    pub book_directory: Option<Cow<'a, str>>,
    pub expiration: Option<u32>,
    #[serde(default)]
    pub flags: u32,
    pub low_limit: Option<IssuedCurrencyAmount<'a>>,
    pub high_limit: Option<IssuedCurrencyAmount<'a>>,
    pub next_page_min: Option<Cow<'a, str>>,
    #[serde(rename = "NFTokens")]
    pub nftokens: Option<Vec<NFTokenMetadata<'a>>>,
    pub previous_page_min: Option<Cow<'a, str>>,
    #[serde(default)]
    pub sequence: u32,
    pub taker_gets: Option<Amount<'a>>,
    pub taker_pays: Option<Amount<'a>>,
    pub xchain_claim_id: Option<Cow<'a, str>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum AffectedNode<'a> {
    #[serde(rename_all = "PascalCase")]
    CreatedNode {
        /// The type of ledger object this node represents.
        ledger_entry_type: LedgerEntryType,
        /// The ID of this ledger entry in the ledger's state tree.
        ledger_index: LedgerIndex<'a>,
        /// The content fields of a newly created ledger entry.
        new_fields: Fields<'a>,
    },
    #[serde(rename_all = "PascalCase")]
    ModifiedNode {
        /// The type of ledger object this node represents.
        ledger_entry_type: LedgerEntryType,
        /// The ID of this ledger entry in the ledger's state tree.
        ledger_index: LedgerIndex<'a>,
        /// The content fields of the ledger entry after changes.
        final_fields: Option<Fields<'a>>,
        /// The previous values for changed fields.
        previous_fields: Option<Fields<'a>>,
        /// The identifying hash of the previous transaction to modify this
        /// ledger entry.
        #[serde(rename = "PreviousTxnID")]
        previous_txn_id: Option<Cow<'a, str>>,
        /// The Ledger Index of the ledger containing the previous transaction.
        previous_txn_lgr_seq: Option<u32>,
    },
    #[serde(rename_all = "PascalCase")]
    DeletedNode {
        /// The type of ledger object this node represents.
        ledger_entry_type: LedgerEntryType,
        /// The ID of this ledger entry in the ledger's state tree.
        ledger_index: LedgerIndex<'a>,
        /// The content fields of the ledger entry after changes.
        final_fields: Fields<'a>,
        /// The previous values for changed fields.
        previous_fields: Option<Fields<'a>>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeType {
    CreatedNode,
    ModifiedNode,
    DeletedNode,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct TransactionMetadata<'a> {
    /// Array of objects describing changes to ledger entries this
    /// transaction made.
    pub affected_nodes: Vec<AffectedNode<'a>>,
    /// The transaction's position within the ledger that included it.
    pub transaction_index: u32,
    /// The transaction's result code.
    pub transaction_result: Cow<'a, str>,
    /// The currency amount actually delivered to the destination for Payment
    /// transactions. Contains "unavailable" for partial payments before
    /// 2014-01-20.
    #[serde(rename = "delivered_amount")]
    pub delivered_amount: Option<Amount<'a>>,
    /// (Optional) NFTokenID for NFTokenMint and NFTokenAcceptOffer
    /// transactions.
    #[serde(rename = "nftoken_id")]
    pub nftoken_id: Option<Cow<'a, str>>,
    /// (Optional) Array of NFTokenIDs for NFTokenCancelOffer transactions.
    #[serde(rename = "nftoken_ids")]
    pub nftoken_ids: Option<Cow<'a, [Cow<'a, str>]>>,
    /// (Optional) OfferID for NFTokenCreateOffer transactions.
    #[serde(rename = "offer_id")]
    pub offer_id: Option<Cow<'a, str>>,
    /// (Optional) MPTokenIssuanceID for MPTokenIssuanceCreate transactions.
    #[serde(rename = "mpt_issuance_id")]
    pub mpt_issuance_id: Option<Cow<'a, str>>,
}

#[cfg(test)]
mod test_serde {
    use crate::models::{ledger::objects::LedgerEntryType, requests::LedgerIndex};

    use super::AffectedNode;

    #[test]
    fn test_deserialize_deleted_node() {
        let json = r#"
            {
                "DeletedNode": {
                    "FinalFields": {
                        "Account": "rHzKtpcB1KC1YuU4PBhk9m2abqrf2kZsfV",
                        "BookDirectory": "623C4C4AD65873DA787AC85A0A1385FE6233B6DE100799474F20E441AE211B08",
                        "BookNode": "0",
                        "Flags": 0,
                        "OwnerNode": "0",
                        "PreviousTxnID": "3C5524789C20AE661CF4985EF694F11128FE267D084CB5E77DBB5FFD78E31E1C",
                        "PreviousTxnLgrSeq": 92173580,
                        "Sequence": 29337056,
                        "TakerGets": "17250658754",
                        "TakerPays": {
                            "currency": "CNY",
                            "issuer": "rJ1adrpGS3xsnQMb9Cw54tWJVFPuSdZHK",
                            "value": "159709.5313591656"
                        }
                    },
                    "LedgerEntryType": "Offer",
                    "LedgerIndex": "D11F69DE8A8CACB130F2E2B9893E5C97B9EE4136759C66C1F3497C8575FF5ED0"
                }
            }
        "#;
        let deleted_node = serde_json::from_str::<super::AffectedNode>(json).unwrap();
        if let AffectedNode::DeletedNode {
            ledger_entry_type,
            ledger_index,
            final_fields,
            previous_fields,
        } = deleted_node
        {
            assert_eq!(ledger_entry_type, LedgerEntryType::Offer);
            assert_eq!(
                ledger_index,
                LedgerIndex::Str(
                    "D11F69DE8A8CACB130F2E2B9893E5C97B9EE4136759C66C1F3497C8575FF5ED0".into()
                )
            );
            assert_eq!(
                final_fields.account,
                Some("rHzKtpcB1KC1YuU4PBhk9m2abqrf2kZsfV".into())
            );
            assert!(previous_fields.is_none());
        } else {
            panic!("expected deleted node")
        }
    }

    #[test]
    fn test_deserialize_modified_node() {
        let json = r#"
            {
                "ModifiedNode": {
                    "FinalFields": {
                        "Account": "rHzKtpcB1KC1YuU4PBhk9m2abqrf2kZsfV",
                        "Balance": "5000542889",
                        "Flags": 0,
                        "OwnerCount": 5,
                        "Sequence": 29337064
                    },
                    "LedgerEntryType": "AccountRoot",
                    "LedgerIndex": "991ED60C316200D33B2EA3E56E505433394DBA7FF5E7ADE8C8850D02BEF1F53A",
                    "PreviousFields": {
                        "Balance": "5000542904",
                        "Sequence": 29337063
                    },
                    "PreviousTxnID": "960FAFAF9CA0465B7475F888946F0D58F9CF49B18F3991D826B03A5025368DDE",
                    "PreviousTxnLgrSeq": 92173588
                }
            }
        "#;
        let modified_node = serde_json::from_str::<super::AffectedNode>(json).unwrap();
        if let AffectedNode::ModifiedNode {
            ledger_entry_type,
            ledger_index,
            final_fields,
            previous_fields,
            previous_txn_id,
            previous_txn_lgr_seq,
        } = modified_node
        {
            assert_eq!(ledger_entry_type, LedgerEntryType::AccountRoot);
            assert_eq!(
                ledger_index,
                LedgerIndex::Str(
                    "991ED60C316200D33B2EA3E56E505433394DBA7FF5E7ADE8C8850D02BEF1F53A".into()
                )
            );
            assert_eq!(
                final_fields.map(|f| f.account),
                Some(Some("rHzKtpcB1KC1YuU4PBhk9m2abqrf2kZsfV".into()))
            );
            assert_eq!(
                previous_fields.map(|f| f.balance),
                Some(Some("5000542904".into()))
            );
            assert_eq!(
                previous_txn_id,
                Some("960FAFAF9CA0465B7475F888946F0D58F9CF49B18F3991D826B03A5025368DDE".into())
            );
            assert_eq!(previous_txn_lgr_seq, Some(92173588));
        } else {
            panic!("expected modified node")
        }
    }

    #[test]
    fn test_deserialize_created_node() {
        let json = r#"
            {
                "CreatedNode": {
                    "LedgerEntryType": "AccountRoot",
                    "LedgerIndex": "991ED60C316200D33B2EA3E56E505433394DBA7FF5E7ADE8C8850D02BEF1F53A",
                    "NewFields": {
                        "Account": "rHzKtpcB1KC1YuU4PBhk9m2abqrf2kZsfV",
                        "Balance": "5000542889",
                        "Flags": 0,
                        "OwnerCount": 5,
                        "Sequence": 29337064
                    }
                }
            }
        "#;
        let created_node = serde_json::from_str::<super::AffectedNode>(json).unwrap();
        if let AffectedNode::CreatedNode {
            ledger_entry_type,
            ledger_index,
            new_fields,
        } = created_node
        {
            assert_eq!(ledger_entry_type, LedgerEntryType::AccountRoot);
            assert_eq!(
                ledger_index,
                LedgerIndex::Str(
                    "991ED60C316200D33B2EA3E56E505433394DBA7FF5E7ADE8C8850D02BEF1F53A".into()
                )
            );
            assert_eq!(
                new_fields.account,
                Some("rHzKtpcB1KC1YuU4PBhk9m2abqrf2kZsfV".into())
            );
        } else {
            panic!("expected created node")
        }
    }

    #[test]
    fn test_transaction_metadata_deserialize() {
        let json = r#"{
            "AffectedNodes": [
                {
                    "ModifiedNode": {
                        "FinalFields": {
                            "Account": "rBTwLga3i2gz3doX6Gva3MgEV8ZCD8jjah",
                            "Balance": "27724423128",
                            "Flags": 0,
                            "OwnerCount": 14,
                            "Sequence": 129693478
                        },
                        "LedgerEntryType": "AccountRoot",
                        "LedgerIndex": "1ED8DDFD80F275CB1CE7F18BB9D906655DE8029805D8B95FB9020B30425821EB",
                        "PreviousFields": {
                            "Balance": "27719423228",
                            "Sequence": 129693477
                        },
                        "PreviousTxnID": "3110F983CDC090750B45C9BFB74B8CE629CA80F57C35612402B2760153822BA5",
                        "PreviousTxnLgrSeq": 86724072
                    }
                },
                {
                    "DeletedNode": {
                        "FinalFields": {
                            "Account": "rPx6Rbh8fStXeP3LwECBisownN2ZyMyzYS",
                            "BookDirectory": "DFA3B6DDAB58C7E8E5D944E736DA4B7046C30E4F460FD9DE4E1566CBCC208000",
                            "BookNode": "0",
                            "Flags": 0,
                            "OwnerNode": "0",
                            "PreviousTxnID": "DCB061EC44BBF73BBC20CE0432E9D8D7C4B8B28ABA8AE5A5BA687476E7A796EF",
                            "PreviousTxnLgrSeq": 86724050,
                            "Sequence": 86586865,
                            "TakerGets": "0",
                            "TakerPays": {
                                "currency": "USD",
                                "issuer": "rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B",
                                "value": "0"
                            }
                        },
                        "LedgerEntryType": "Offer",
                        "LedgerIndex": "348AF66EBD872FBF2BD23085D3FB4A200E15509451475027C4A5EE8D8B77C623"
                    }
                }
            ],
            "TransactionIndex": 5,
            "TransactionResult": "tesSUCCESS"
        }"#;

        let metadata: super::TransactionMetadata = serde_json::from_str(json).unwrap();

        assert_eq!(metadata.transaction_index, 5);
        assert_eq!(metadata.transaction_result, "tesSUCCESS");
        assert_eq!(metadata.affected_nodes.len(), 2);

        // Test first affected node (ModifiedNode)
        let first_node = &metadata.affected_nodes[0];
        if let AffectedNode::ModifiedNode {
            ledger_entry_type,
            ledger_index,
            previous_txn_lgr_seq,
            ..
        } = &first_node
        {
            assert_eq!(*ledger_entry_type, LedgerEntryType::AccountRoot);
            assert_eq!(
                *ledger_index,
                LedgerIndex::Str(
                    "1ED8DDFD80F275CB1CE7F18BB9D906655DE8029805D8B95FB9020B30425821EB".into()
                )
            );
            assert_eq!(*previous_txn_lgr_seq, Some(86724072));
        } else {
            panic!("Expected ModifiedNode");
        }

        // Test second affected node (DeletedNode)
        let second_node = &metadata.affected_nodes[1];
        if let AffectedNode::DeletedNode {
            ledger_entry_type,
            ledger_index,
            ..
        } = &second_node
        {
            assert_eq!(*ledger_entry_type, LedgerEntryType::Offer);
            assert_eq!(
                *ledger_index,
                LedgerIndex::Str(
                    "348AF66EBD872FBF2BD23085D3FB4A200E15509451475027C4A5EE8D8B77C623".into()
                )
            );
        } else {
            panic!("Expected DeletedNode");
        }
    }
}
