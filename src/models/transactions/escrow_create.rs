use alloc::borrow::Cow;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::amount::XRPAmount;
use crate::models::transactions::CommonFields;
use crate::models::{
    transactions::{Memo, Signer, Transaction, TransactionType},
    Model, ValidateCurrencies,
};
use crate::models::{FlagCollection, NoFlags, XRPLModelException, XRPLModelResult};

use super::CommonTransactionBuilder;

/// Creates an Escrow, which requests XRP until the escrow process either finishes or is canceled.
///
/// See EscrowCreate:
/// `<https://xrpl.org/docs/references/protocol/transactions/types/escrowcreate>`
#[skip_serializing_none]
#[derive(
    Debug,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Clone,
    xrpl_rust_macros::ValidateCurrencies,
)]
#[serde(rename_all = "PascalCase")]
pub struct EscrowCreate<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(flatten)]
    pub common_fields: CommonFields<'a, NoFlags>,
    /// Amount of XRP, in drops, to deduct from the sender's balance and escrow.
    /// Once escrowed, the XRP can either go to the Destination address
    /// (after the FinishAfter time) or returned to the sender (after the CancelAfter time).
    pub amount: XRPAmount<'a>,
    /// Address to receive escrowed XRP.
    pub destination: Cow<'a, str>,
    /// Arbitrary tag to further specify the destination for this escrowed
    /// payment, such as a hosted recipient at the destination address.
    pub destination_tag: Option<u32>,
    /// The time, in seconds since the Ripple Epoch, when this
    /// escrow expires. This value is immutable; the funds can
    /// only be returned to the sender after this time.
    pub cancel_after: Option<u32>,
    /// The time, in seconds since the Ripple Epoch, when the escrowed XRP
    /// can be released to the recipient. This value is immutable, and the
    /// funds can't be accessed until this time.
    pub finish_after: Option<u32>,
    /// Hex value representing a PREIMAGE-SHA-256 crypto-condition.
    /// The funds can only be delivered to the recipient if this
    /// condition is fulfilled. If the condition is not fulfilled
    /// before the expiration time specified in the CancelAfter
    /// field, the XRP can only revert to the sender.
    pub condition: Option<Cow<'a, str>>,
    /// Smart-escrow WebAssembly bytecode (hex-encoded), evaluated by rippled
    /// on `EscrowFinish` to decide whether the escrow releases. Added by the
    /// [XLS-100 Smart Escrows amendment](https://xls.xrpl.org/xls/XLS-0100-smart-escrows.html).
    /// Optional — the field is only meaningful on a rippled build that
    /// enables the amendment.
    pub bytecode: Option<Cow<'a, str>>,
    /// Opaque contract-owned state (hex-encoded) accessible to the smart-escrow
    /// bytecode at finish time. Also part of XLS-100; optional.
    pub data: Option<Cow<'a, str>>,
}

impl<'a> Model for EscrowCreate<'a> {
    fn get_errors(&self) -> XRPLModelResult<()> {
        self._get_finish_after_error()?;
        self.validate_currencies()
    }
}

impl<'a> Transaction<'a, NoFlags> for EscrowCreate<'a> {
    fn get_transaction_type(&self) -> &TransactionType {
        self.common_fields.get_transaction_type()
    }

    fn get_common_fields(&self) -> &CommonFields<'_, NoFlags> {
        self.common_fields.get_common_fields()
    }

    fn get_mut_common_fields(&mut self) -> &mut CommonFields<'a, NoFlags> {
        self.common_fields.get_mut_common_fields()
    }
}

impl<'a> CommonTransactionBuilder<'a, NoFlags> for EscrowCreate<'a> {
    fn get_mut_common_fields(&mut self) -> &mut CommonFields<'a, NoFlags> {
        &mut self.common_fields
    }

    fn into_self(self) -> Self {
        self
    }
}

impl<'a> EscrowCreate<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        account: Cow<'a, str>,
        account_txn_id: Option<Cow<'a, str>>,
        fee: Option<XRPAmount<'a>>,
        last_ledger_sequence: Option<u32>,
        memos: Option<Vec<Memo>>,
        sequence: Option<u32>,
        signers: Option<Vec<Signer>>,
        source_tag: Option<u32>,
        ticket_sequence: Option<u32>,
        amount: XRPAmount<'a>,
        destination: Cow<'a, str>,
        cancel_after: Option<u32>,
        condition: Option<Cow<'a, str>>,
        destination_tag: Option<u32>,
        finish_after: Option<u32>,
        bytecode: Option<Cow<'a, str>>,
        data: Option<Cow<'a, str>>,
    ) -> Self {
        Self {
            common_fields: CommonFields::new(
                account,
                TransactionType::EscrowCreate,
                account_txn_id,
                fee,
                Some(FlagCollection::default()),
                last_ledger_sequence,
                memos,
                None,
                sequence,
                signers,
                None,
                source_tag,
                ticket_sequence,
                None,
            ),
            amount,
            destination,
            destination_tag,
            cancel_after,
            finish_after,
            condition,
            bytecode,
            data,
        }
    }

    pub fn with_destination_tag(mut self, destination_tag: u32) -> Self {
        self.destination_tag = Some(destination_tag);
        self
    }

    pub fn with_cancel_after(mut self, cancel_after: u32) -> Self {
        self.cancel_after = Some(cancel_after);
        self
    }

    pub fn with_finish_after(mut self, finish_after: u32) -> Self {
        self.finish_after = Some(finish_after);
        self
    }

    pub fn with_condition(mut self, condition: Cow<'a, str>) -> Self {
        self.condition = Some(condition);
        self
    }

    /// Attach smart-escrow bytecode (hex-encoded WebAssembly). Requires the
    /// XLS-100 Smart Escrows amendment on the target network.
    pub fn with_bytecode(mut self, bytecode: Cow<'a, str>) -> Self {
        self.bytecode = Some(bytecode);
        self
    }

    /// Attach opaque contract-owned state for the smart-escrow contract to
    /// read at finish time. Requires XLS-100.
    pub fn with_data(mut self, data: Cow<'a, str>) -> Self {
        self.data = Some(data);
        self
    }
}

impl<'a> EscrowCreateError for EscrowCreate<'a> {
    fn _get_finish_after_error(&self) -> XRPLModelResult<()> {
        if let (Some(finish_after), Some(cancel_after)) = (self.finish_after, self.cancel_after) {
            if finish_after >= cancel_after {
                Err(XRPLModelException::ValueBelowValue {
                    field1: "cancel_after".into(),
                    field2: "finish_after".into(),
                    field1_val: cancel_after,
                    field2_val: finish_after,
                })
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}

pub trait EscrowCreateError {
    fn _get_finish_after_error(&self) -> XRPLModelResult<()>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Model;

    #[test]
    fn test_cancel_after_error() {
        let escrow_create = EscrowCreate {
            common_fields: CommonFields {
                account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb".into(),
                transaction_type: TransactionType::EscrowCreate,
                ..Default::default()
            },
            amount: XRPAmount::from("100000000"),
            destination: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb".into(),
            cancel_after: Some(13298498),
            finish_after: Some(14359039),
            ..Default::default()
        };

        assert!(escrow_create.get_errors().is_err());
    }

    #[test]
    fn test_valid_timing() {
        let escrow_create = EscrowCreate {
            common_fields: CommonFields {
                account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb".into(),
                transaction_type: TransactionType::EscrowCreate,
                ..Default::default()
            },
            amount: XRPAmount::from("100000000"),
            destination: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb".into(),
            cancel_after: Some(14359039),
            finish_after: Some(13298498),
            ..Default::default()
        };

        assert!(escrow_create.get_errors().is_ok());
    }

    #[test]
    fn test_serde() {
        let default_txn = EscrowCreate {
            common_fields: CommonFields {
                account: "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn".into(),
                transaction_type: TransactionType::EscrowCreate,
                source_tag: Some(11747),
                signing_pub_key: Some("".into()),
                ..Default::default()
            },
            amount: XRPAmount::from("10000"),
            destination: "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW".into(),
            destination_tag: Some(23480),
            cancel_after: Some(533257958),
            finish_after: Some(533171558),
            condition: Some(
                "A0258020E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855810100"
                    .into(),
            ),
            bytecode: None,
            data: None,
        };

        let default_json_str = r#"{"Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","TransactionType":"EscrowCreate","Flags":0,"SigningPubKey":"","SourceTag":11747,"Amount":"10000","Destination":"rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW","DestinationTag":23480,"CancelAfter":533257958,"FinishAfter":533171558,"Condition":"A0258020E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855810100"}"#;

        let default_json_value = serde_json::to_value(default_json_str).unwrap();
        let serialized_string = serde_json::to_string(&default_txn).unwrap();
        let serialized_value = serde_json::to_value(&serialized_string).unwrap();
        assert_eq!(serialized_value, default_json_value);

        let deserialized: EscrowCreate = serde_json::from_str(default_json_str).unwrap();
        assert_eq!(default_txn, deserialized);
    }

    #[test]
    fn test_builder_pattern() {
        let escrow_create = EscrowCreate {
            common_fields: CommonFields {
                account: "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn".into(),
                transaction_type: TransactionType::EscrowCreate,
                ..Default::default()
            },
            amount: XRPAmount::from("10000"),
            destination: "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW".into(),
            ..Default::default()
        }
        .with_destination_tag(23480)
        .with_cancel_after(533257958)
        .with_finish_after(533171558)
        .with_condition(
            "A0258020E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855810100".into(),
        )
        .with_source_tag(11747)
        .with_fee("12".into())
        .with_sequence(123);

        assert_eq!(escrow_create.amount.0, "10000");
        assert_eq!(
            escrow_create.destination,
            "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW"
        );
        assert_eq!(escrow_create.destination_tag, Some(23480));
        assert_eq!(escrow_create.cancel_after, Some(533257958));
        assert_eq!(escrow_create.finish_after, Some(533171558));
        assert!(escrow_create.condition.is_some());
        assert_eq!(escrow_create.common_fields.source_tag, Some(11747));
        assert_eq!(escrow_create.common_fields.fee.as_ref().unwrap().0, "12");
        assert_eq!(escrow_create.common_fields.sequence, Some(123));
        assert!(escrow_create.get_errors().is_ok());
    }

    #[test]
    fn test_default() {
        let escrow_create = EscrowCreate {
            common_fields: CommonFields {
                account: "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn".into(),
                transaction_type: TransactionType::EscrowCreate,
                ..Default::default()
            },
            amount: XRPAmount::from("10000"),
            destination: "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW".into(),
            ..Default::default()
        };

        assert_eq!(
            escrow_create.common_fields.account,
            "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn"
        );
        assert_eq!(
            escrow_create.common_fields.transaction_type,
            TransactionType::EscrowCreate
        );
        assert_eq!(escrow_create.amount.0, "10000");
        assert_eq!(
            escrow_create.destination,
            "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW"
        );
        assert!(escrow_create.destination_tag.is_none());
        assert!(escrow_create.cancel_after.is_none());
        assert!(escrow_create.finish_after.is_none());
        assert!(escrow_create.condition.is_none());
        assert!(escrow_create.bytecode.is_none());
        assert!(escrow_create.data.is_none());
    }

    /// Verifies the two XLS-100 fields (`Bytecode`, `Data`) round-trip
    /// through serde and set correctly via the builder helpers. Uses the
    /// smallest valid WASM module that exports `escrow_finish() -> i32`
    /// returning 1 (46 bytes) as the bytecode payload.
    #[test]
    fn test_smart_escrow_fields() {
        const RETURN_1_WASM_HEX: &str = "0061736D010000000105016000017F030201000711010D657363726F775F66696E69736800000A0601040041010B";

        let escrow_create = EscrowCreate {
            common_fields: CommonFields {
                account: "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn".into(),
                transaction_type: TransactionType::EscrowCreate,
                ..Default::default()
            },
            amount: XRPAmount::from("10000"),
            destination: "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW".into(),
            ..Default::default()
        }
        .with_bytecode(RETURN_1_WASM_HEX.into())
        .with_data("DEADBEEF".into());

        assert_eq!(escrow_create.bytecode.as_deref(), Some(RETURN_1_WASM_HEX));
        assert_eq!(escrow_create.data.as_deref(), Some("DEADBEEF"));

        // Serde: PascalCase per the struct-level attribute.
        let serialized = serde_json::to_string(&escrow_create).unwrap();
        assert!(
            serialized.contains(&alloc::format!("\"Bytecode\":\"{RETURN_1_WASM_HEX}\"")),
            "serialized: {serialized}"
        );
        assert!(serialized.contains("\"Data\":\"DEADBEEF\""));

        let deserialized: EscrowCreate = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.bytecode.as_deref(), Some(RETURN_1_WASM_HEX));
        assert_eq!(deserialized.data.as_deref(), Some("DEADBEEF"));

        // Both fields are optional and validation doesn't touch them.
        assert!(escrow_create.get_errors().is_ok());
    }
}
