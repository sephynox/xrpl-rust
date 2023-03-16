use crate::models::ledger::LedgerEntryType;
use crate::models::Model;

use alloc::vec::Vec;
use derive_new::new;
use serde::{ser::SerializeMap, Deserialize, Serialize};

use crate::serialize_with_tag;
use serde_with::skip_serializing_none;

serialize_with_tag! {
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, new, Default)]
pub struct DisabledValidator<'a> {
    pub first_ledger_sequence: u32,
    pub public_key: &'a str,
}
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NegativeUNL<'a> {
    pub ledger_entry_type: LedgerEntryType,
    /// A bit-map of boolean flags. No flags are defined for the NegativeUNL object type, so this
    /// value is always 0.
    pub flags: u32,
    /// The object ID of a single object to retrieve from the ledger, as a
    /// 64-character (256-bit) hexadecimal string.
    #[serde(rename = "index")]
    pub index: &'a str,
    pub disabled_validators: Option<Vec<DisabledValidator<'a>>>,
    pub validator_to_disable: Option<&'a str>,
    pub validator_to_re_enable: Option<&'a str>,
}

impl<'a> Default for NegativeUNL<'a> {
    fn default() -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::NegativeUNL,
            flags: Default::default(),
            index: Default::default(),
            disabled_validators: Default::default(),
            validator_to_disable: Default::default(),
            validator_to_re_enable: Default::default(),
        }
    }
}

impl<'a> Model for NegativeUNL<'a> {}

impl<'a> NegativeUNL<'a> {
    pub fn new(
        index: &'a str,
        disabled_validators: Option<Vec<DisabledValidator<'a>>>,
        validator_to_disable: Option<&'a str>,
        validator_to_re_enable: Option<&'a str>,
    ) -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::NegativeUNL,
            flags: 0,
            index,
            disabled_validators,
            validator_to_disable,
            validator_to_re_enable,
        }
    }
}

#[cfg(test)]
mod test_serde {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_serialize() {
        let negative_unl = NegativeUNL::new(
            "2E8A59AA9D3B5B186B0B9E0F62E6C02587CA74A4D778938E957B6357D364B244",
            Some(vec![DisabledValidator::new(
                1609728,
                "ED6629D456285AE3613B285F65BBFF168D695BA3921F309949AFCD2CA7AFEC16FE",
            )]),
            None,
            None,
        );
        let negative_unl_json = serde_json::to_string(&negative_unl).unwrap();
        let actual = negative_unl_json.as_str();
        let expected = r#"{"LedgerEntryType":"NegativeUNL","Flags":0,"index":"2E8A59AA9D3B5B186B0B9E0F62E6C02587CA74A4D778938E957B6357D364B244","DisabledValidators":[{"DisabledValidator":{"FirstLedgerSequence":1609728,"PublicKey":"ED6629D456285AE3613B285F65BBFF168D695BA3921F309949AFCD2CA7AFEC16FE"}}]}"#;

        assert_eq!(expected, actual);
    }
}
