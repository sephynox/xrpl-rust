use core::marker::PhantomData;

use alloc::{borrow::Cow, string::ToString, vec::Vec};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::{transactions::metadata::TransactionMetadata, Amount, XRPAmount};

/// See Subscribe:
/// `<https://xrpl.org/subscribe.html#subscribe>`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Subscribe<'a> {
    /// Keep the lifetime parameter consistent with other result types
    #[serde(skip)]
    phantom: PhantomData<&'a ()>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Stream<'a, T = Value> {
    /// Represents a subscription to a specific ledger.
    #[serde(untagged)]
    LedgerClosed(LedgerClosedStream<'a>),
    /// Represents a subscription to a specific transaction.
    #[serde(untagged)]
    Transaction(TransactionStream<'a, T>),
    /// Represents a subscription to a specific order book.
    #[serde(untagged)]
    BookChanges(BookChangesStream<'a>),
    /// Fallback for any other type not explicitly defined
    /// This can be used to handle future extensions or custom types.
    #[serde(untagged)]
    Other(Value),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum StreamType {
    /// Represents a subscription to a specific ledger.
    LedgerClosed,
    /// Represents a subscription to a specific transaction.
    Transaction,
    /// Represents a subscription to a specific order book.
    BookChanges,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BaseStream {
    /// The type of the stream.
    pub r#type: StreamType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LedgerClosedStream<'a> {
    /// The base fields for a stream.
    #[serde(flatten)]
    pub base: BaseStream,
    /// The base fee for the ledger.
    pub fee_base: u64,
    /// The reference fee for the ledger.
    pub fee_ref: Option<XRPAmount<'a>>,
    /// The hash of the ledger.
    pub ledger_hash: Cow<'a, str>,
    /// The index of the ledger.
    pub ledger_index: u64,
    /// The time of the ledger.
    pub ledger_time: u64,
    /// The base reserve for the ledger.
    pub reserve_base: XRPAmount<'a>,
    /// The increment reserve for the ledger.
    pub reserve_inc: XRPAmount<'a>,
    /// The count of transactions in the ledger.
    pub txn_count: u32,
    /// The validated ledgers in the format "start-end".
    pub validated_ledgers: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionStream<'a, T> {
    #[serde(untagged)]
    V1(TransactionStreamV1<'a, T>),
    #[serde(untagged)]
    V2(TransactionStreamV2<'a, T>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransactionStreamV1<'a, T> {
    /// The base fields for a transaction stream.
    #[serde(flatten)]
    pub base: BaseStream,
    pub engine_result: Cow<'a, str>,
    pub engine_result_code: u32,
    pub engine_result_message: Cow<'a, str>,
    pub ledger_current_index: Option<u64>,
    pub ledger_hash: Option<Cow<'a, str>>,
    pub ledger_index: Option<u64>,
    pub meta: Option<TransactionMetadata<'a>>,
    pub transaction: T,
    pub validated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransactionStreamV2<'a, T> {
    /// The base fields for a transaction stream.
    #[serde(flatten)]
    pub base: BaseStream,
    /// The close time in ISO format.
    pub close_time_iso: Cow<'a, str>,
    /// The engine result of the transaction.
    pub engine_result: Cow<'a, str>,
    /// The engine result code.
    pub engine_result_code: i32,
    /// The engine result message.
    pub engine_result_message: Cow<'a, str>,
    /// The transaction hash.
    pub hash: Cow<'a, str>,
    /// The ledger current index.
    pub ledger_current_index: Option<u64>,
    /// The ledger hash.
    pub ledger_hash: Option<Cow<'a, str>>,
    /// The ledger index.
    pub ledger_index: Option<u64>,
    /// Metadata about the transaction.
    pub meta: Option<TransactionMetadata<'a>>,
    /// The status of the transaction.
    pub status: Cow<'a, str>,
    /// The transaction JSON.
    pub tx_json: T,
    /// Whether the transaction is validated.
    pub validated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BookChangesStream<'a> {
    /// The base fields for a book changes stream.
    #[serde(flatten)]
    pub base: BaseStream,
    /// The ledger index.
    pub ledger_index: u64,
    /// The ledger hash.
    pub ledger_hash: Cow<'a, str>,
    /// The ledger time.
    pub ledger_time: u64,
    /// The changes in the order book.
    pub changes: Vec<BookChange<'a>>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct BookChange<'a> {
    /// The currency A in the book change.
    pub currency_a: Cow<'a, str>,
    /// The currency B in the book change.
    pub currency_b: Cow<'a, str>,
    /// The volume of currency A.
    pub volume_a: Amount<'a>,
    /// The volume of currency B.
    pub volume_b: Amount<'a>,
    /// The high price in the book change.
    pub high: Cow<'a, str>,
    /// The low price in the book change.
    pub low: Cow<'a, str>,
    /// The open price in the book change.
    pub open: Cow<'a, str>,
    /// The close price in the book change.
    pub close: Cow<'a, str>,
}

impl<'de, 'a> Deserialize<'de> for BookChange<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        Self: 'a,
        D: serde::Deserializer<'de>,
    {
        let mut map = serde_json::Map::deserialize(deserializer)?;
        let currency_a_binding = map
            .remove("currency_a")
            .ok_or_else(|| serde::de::Error::missing_field("currency_a"))?;
        let currency_a = currency_a_binding
            .as_str()
            .ok_or_else(|| {
                serde::de::Error::invalid_type(
                    serde::de::Unexpected::Other("not a string"),
                    &"string",
                )
            })?
            .to_string();
        let currency_b_binding = map
            .remove("currency_b")
            .ok_or_else(|| serde::de::Error::missing_field("currency_b"))?;
        let currency_b = currency_b_binding
            .as_str()
            .ok_or_else(|| {
                serde::de::Error::invalid_type(
                    serde::de::Unexpected::Other("not a string"),
                    &"string",
                )
            })?
            .to_string();
        let volume_a_binding = map
            .remove("volume_a")
            .ok_or_else(|| serde::de::Error::missing_field("volume_a"))?;
        let volume_a = volume_a_binding
            .as_str()
            .ok_or_else(|| {
                serde::de::Error::invalid_type(
                    serde::de::Unexpected::Other("not a string"),
                    &"string",
                )
            })?
            .to_string();
        let volume_b_binding = map
            .remove("volume_b")
            .ok_or_else(|| serde::de::Error::missing_field("volume_b"))?;
        let volume_b = volume_b_binding
            .as_str()
            .ok_or_else(|| {
                serde::de::Error::invalid_type(
                    serde::de::Unexpected::Other("not a string"),
                    &"string",
                )
            })?
            .to_string();
        let high_binding = map
            .remove("high")
            .ok_or_else(|| serde::de::Error::missing_field("high"))?;
        let high = high_binding.as_str().ok_or_else(|| {
            serde::de::Error::invalid_type(serde::de::Unexpected::Other("not a string"), &"string")
        })?;
        let low_binding = map
            .remove("low")
            .ok_or_else(|| serde::de::Error::missing_field("low"))?;
        let low = low_binding.as_str().ok_or_else(|| {
            serde::de::Error::invalid_type(serde::de::Unexpected::Other("not a string"), &"string")
        })?;
        let open_binding = map
            .remove("open")
            .ok_or_else(|| serde::de::Error::missing_field("open"))?;
        let open = open_binding.as_str().ok_or_else(|| {
            serde::de::Error::invalid_type(serde::de::Unexpected::Other("not a string"), &"string")
        })?;
        let close_binding = map
            .remove("close")
            .ok_or_else(|| serde::de::Error::missing_field("close"))?;
        let close = close_binding.as_str().ok_or_else(|| {
            serde::de::Error::invalid_type(serde::de::Unexpected::Other("not a string"), &"string")
        })?;

        Ok(BookChange {
            currency_a: currency_a.clone().into(),
            currency_b: currency_b.clone().into(),
            volume_a: Amount::try_from((currency_a.clone().into(), volume_a.clone().into()))
                .map_err(serde::de::Error::custom)?,
            volume_b: Amount::try_from((currency_b.clone().into(), volume_b.clone().into()))
                .map_err(serde::de::Error::custom)?,
            high: high.to_string().into(),
            low: low.to_string().into(),
            open: open.to_string().into(),
            close: close.to_string().into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::models::transactions::payment::Payment;

    use super::*;

    #[test]
    fn test_serde_ledger_stream() {
        let json = r#"{
            "type": "ledgerClosed",
            "fee_base": 10,
            "fee_ref": 10,
            "ledger_hash": "687F604EF6B2F67319E8DCC8C66EF49D84D18A1E18F948421FC24D2C7C3DB464",
            "ledger_index": 7125358,
            "ledger_time": 455751310,
            "reserve_base": 20000000,
            "reserve_inc": 5000000,
            "txn_count": 7,
            "validated_ledgers": "32570-7125358"
        }"#;

        let ledger_stream: Stream = serde_json::from_str(json).unwrap();
        let stream = Stream::LedgerClosed(LedgerClosedStream {
            base: BaseStream {
                r#type: StreamType::LedgerClosed,
            },
            fee_base: 10,
            fee_ref: Some(XRPAmount::from(10)),
            ledger_hash: "687F604EF6B2F67319E8DCC8C66EF49D84D18A1E18F948421FC24D2C7C3DB464".into(),
            ledger_index: 7125358,
            ledger_time: 455751310,
            reserve_base: XRPAmount::from(20000000),
            reserve_inc: XRPAmount::from(5000000),
            txn_count: 7,
            validated_ledgers: Some("32570-7125358".into()),
        });
        assert_eq!(ledger_stream, stream);
    }

    #[test]
    fn test_serde_transaction_stream_v1() {
        let json = r#"{
            "type": "transaction",
            "transaction_hash": "6489E52A909208E371ACE82E19CAE59896C7F8BA40E7C36C5B8AA3C451914BED"
        }"#;
    }

    #[test]
    fn test_serde_transaction_stream_v2() {
        let json = r#"{
            "close_time_iso": "2024-11-01T23:59:01Z",
            "engine_result": "tesSUCCESS",
            "engine_result_code": 0,
            "engine_result_message": "The transaction was applied. Only final in a validated ledger.",
            "hash": "6489E52A909208E371ACE82E19CAE59896C7F8BA40E7C36C5B8AA3C451914BED",
            "ledger_hash": "0B6F44849E6D702D0CFB447FDBD7B603C269E9EEECE9176882EF376E0C9DFF6A",
            "ledger_index": 1969852,
            "meta": {
                "AffectedNodes": [
                {
                    "ModifiedNode": {
                    "FinalFields": {
                        "Account": "rH3PxjJPrrkvsATddBXkayjAyWR8xigaE8",
                        "Balance": "39999964",
                        "Flags": 0,
                        "OwnerCount": 0,
                        "Sequence": 1969812
                    },
                    "LedgerEntryType": "AccountRoot",
                    "LedgerIndex": "EDE60B24659BCC06CCE1EA2804A4A202F1C88155CEAED9C140833C0C39100617",
                    "PreviousFields": {
                        "Balance": "59999976",
                        "Sequence": 1969811
                    },
                    "PreviousTxnID": "1DBC93373D47794A684A5013178D0EBE10E6641D7C262BF20151B0E19156FF79",
                    "PreviousTxnLgrSeq": 1969843
                    }
                },
                {
                    "ModifiedNode": {
                    "FinalFields": {
                        "Account": "rfdGuuVnq9juqWDV4W3LoLiNcW8g2hAXhN",
                        "Balance": "160000000",
                        "Flags": 0,
                        "OwnerCount": 0,
                        "Sequence": 1969810
                    },
                    "LedgerEntryType": "AccountRoot",
                    "LedgerIndex": "F7D350FB54C5BBA734AE574EE6BF7A9294E11F9B75413972F98846AFC587C62C",
                    "PreviousFields": {
                        "Balance": "140000000"
                    },
                    "PreviousTxnID": "1DBC93373D47794A684A5013178D0EBE10E6641D7C262BF20151B0E19156FF79",
                    "PreviousTxnLgrSeq": 1969843
                    }
                }
                ],
                "TransactionIndex": 4,
                "TransactionResult": "tesSUCCESS",
                "delivered_amount": "20000000"
            },
            "status": "closed",
            "tx_json": {
                "Account": "rH3PxjJPrrkvsATddBXkayjAyWR8xigaE8",
                "DeliverMax": "20000000",
                "Destination": "rfdGuuVnq9juqWDV4W3LoLiNcW8g2hAXhN",
                "Fee": "12",
                "Flags": 0,
                "LastLedgerSequence": 1969870,
                "Sequence": 1969811,
                "SigningPubKey": "ED0761CDA5507784F6CEB445DE2343F861DD5EC7A869F75B08C7E8F29A947AD9FC",
                "TransactionType": "Payment",
                "TxnSignature": "20D5447ED7095BCCC3D42EA1955600D97D791811072E93D2A358AD9FB258C3A7F004974039D25708F5AE598C78F85B688DD586158F7E9C13AE0F30CC18E3390D",
                "date": 783820741
            },
            "type": "transaction",
            "validated": true
        }"#;

        let transaction_stream: Stream = serde_json::from_str(json).unwrap();
    }

    #[test]
    fn test_serde_book_changes_stream() {
        let json = r#"{
            "type": "bookChanges",
            "ledger_index": 88530525,
            "ledger_hash": "E2F24290E1714C842D34A1057E6D6B7327C7DDD310263AFBC67CA8EFED7A331B",
            "ledger_time": 771099232,
            "changes": [
                {
                "currency_a": "XRP_drops",
                "currency_b": "rKiCet8SdvWxPXnAgYarFUXMh1zCPz432Y/USD",
                "volume_a": "23020993",
                "volume_b": "11.51049687275246",
                "high": "1999999.935232603",
                "low": "1999999.935232603",
                "open": "1999999.935232603",
                "close": "1999999.935232603"
                },
                {
                "currency_a": "XRP_drops",
                "currency_b": "rRbiKwcueo6MchUpMFDce9XpDwHhRLPFo/43525950544F0000000000000000000000000000",
                "volume_a": "28062",
                "volume_b": "0.000643919229004",
                "high": "43580000.00000882",
                "low": "43580000.00000882",
                "open": "43580000.00000882",
                "close": "43580000.00000882"
                },
                {
                "currency_a": "XRP_drops",
                "currency_b": "rcEGREd8NmkKRE8GE424sksyt1tJVFZwu/5553444300000000000000000000000000000000",
                "volume_a": "147797392",
                "volume_b": "70.41143840513008",
                "high": "2099053.724049922",
                "low": "2099053.724049922",
                "open": "2099053.724049922",
                "close": "2099053.724049922"
                },
                {
                "currency_a": "XRP_drops",
                "currency_b": "rcRzGWq6Ng3jeYhqnmM4zcWcUh69hrQ8V/LTC",
                "volume_a": "350547165",
                "volume_b": "2.165759976556748",
                "high": "162573356.3100158",
                "low": "160134763.7403094",
                "open": "162573356.3100158",
                "close": "160134763.7403094"
                },
                {
                "currency_a": "XRP_drops",
                "currency_b": "rchGBxcD1A1C2tdxF6papQYZ8kjRKMYcL/BTC",
                "volume_a": "352373535",
                "volume_b": "0.00249291478138912",
                "high": "1413500174054660e-4",
                "low": "1413499999999996e-4",
                "open": "1413500174054660e-4",
                "close": "1413499999999996e-4"
                },
                {
                "currency_a": "XRP_drops",
                "currency_b": "rcvxE9PS9YBwxtGg1qNeewV6ZB3wGubZq/5553445400000000000000000000000000000000",
                "volume_a": "8768045",
                "volume_b": "4.193604075536",
                "high": "2090813.734932601",
                "low": "2090813.734932601",
                "open": "2090813.734932601",
                "close": "2090813.734932601"
                },
                {
                "currency_a": "XRP_drops",
                "currency_b": "rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq/USD",
                "volume_a": "28113",
                "volume_b": "0.013405652999",
                "high": "2097100.380123005",
                "low": "2097100.380123005",
                "open": "2097100.380123005",
                "close": "2097100.380123005"
                },
                {
                "currency_a": "r3dVizzUAS3U29WKaaSALqkieytA2LCoRe/58434F5245000000000000000000000000000000",
                "currency_b": "rcoreNywaoz2ZCQ8Lg2EbSLnGuRBmun6D/434F524500000000000000000000000000000000",
                "volume_a": "75.626516003375",
                "volume_b": "63.022096669479",
                "high": "1.200000000000003",
                "low": "1.200000000000003",
                "open": "1.200000000000003",
                "close": "1.200000000000003"
                },
                {
                "currency_a": "rKiCet8SdvWxPXnAgYarFUXMh1zCPz432Y/CNY",
                "currency_b": "rKiCet8SdvWxPXnAgYarFUXMh1zCPz432Y/USD",
                "volume_a": "83.9115222024",
                "volume_b": "11.51049687275",
                "high": "7.290000000004561",
                "low": "7.290000000004561",
                "open": "7.290000000004561",
                "close": "7.290000000004561"
                },
                {
                "currency_a": "rcRzGWq6Ng3jeYhqnmM4zcWcUh69hrQ8V/LTC",
                "currency_b": "rchGBxcD1A1C2tdxF6papQYZ8kjRKMYcL/BTC",
                "volume_a": "0.64167647147626",
                "volume_b": "0.00073047551165797",
                "high": "878.4366638381051",
                "low": "878.4366638381051",
                "open": "878.4366638381051",
                "close": "878.4366638381051"
                },
                {
                "currency_a": "rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq/USD",
                "currency_b": "rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B/USD",
                "volume_a": "0.013432464305",
                "volume_b": "0.013566788948",
                "high": "0.9900990099046391",
                "low": "0.9900990099046391",
                "open": "0.9900990099046391",
                "close": "0.9900990099046391"
                }
            ]
        }"#;

        let book_changes_stream: Stream = serde_json::from_str(json).unwrap();
    }
}
