use crate::models::ledger::LedgerEntryType;
use crate::models::{Amount, Model};
use alloc::borrow::Cow;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;
use strum_macros::{AsRefStr, Display, EnumIter};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Majority<'a> {
    #[serde(rename_all = "PascalCase")]
    Majority { amendment: &'a str, close_time: u32 },
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Amendments<'a> {
    ledger_entry_type: LedgerEntryType,
    amendments: Vec<Cow<'a, str>>,
    flags: u32,
    #[serde(borrow = "'a")] // lifetime issue
    majorities: Vec<Majority<'a>>,
    #[serde(rename = "index")]
    index: &'a str,
}

impl<'a> Model for Amendments<'a> {}

impl<'a> Default for Amendments<'a> {
    fn default() -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::Amendments,
            amendments: Default::default(),
            flags: Default::default(),
            majorities: Default::default(),
            index: Default::default(),
        }
    }
}

impl<'a> Amendments<'a> {
    pub fn new(
        amendments: Vec<Cow<'a, str>>,
        flags: u32,
        majorities: Vec<Majority<'a>>,
        index: &'a str,
    ) -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::Amendments,
            amendments,
            flags,
            majorities,
            index,
        }
    }
}

#[cfg(test)]
mod test_serde {
    use crate::models::ledger::{Amendments, Majority};
    use alloc::borrow::Cow;
    use alloc::vec;

    #[test]
    fn test_serialize() {
        let amendments = Amendments::new(
            vec![
                Cow::from("42426C4D4F1009EE67080A9B7965B44656D7714D104A72F9B4369F97ABF044EE"),
                Cow::from("4C97EBA926031A7CF7D7B36FDE3ED66DDA5421192D63DE53FFB46E43B9DC8373"),
                Cow::from("6781F8368C4771B83E8B821D88F580202BCB4228075297B19E4FDC5233F1EFDC"),
                Cow::from("740352F2412A9909880C23A559FCECEDA3BE2126FED62FC7660D628A06927F11"),
            ],
            0,
            vec![Majority::Majority {
                amendment: "1562511F573A19AE9BD103B5D6B9E01B3B46805AEC5D3C4805C902B514399146",
                close_time: 535589001,
            }],
            "7DB0788C020F02780A673DC74757F23823FA3014C1866E72CC4CD8B226CD6EF4",
        );
        let amendments_json = serde_json::to_string(&amendments).unwrap();
        let actual = amendments_json.as_str();
        let expected = r#"{"LedgerEntryType":"Amendments","Amendments":["42426C4D4F1009EE67080A9B7965B44656D7714D104A72F9B4369F97ABF044EE","4C97EBA926031A7CF7D7B36FDE3ED66DDA5421192D63DE53FFB46E43B9DC8373","6781F8368C4771B83E8B821D88F580202BCB4228075297B19E4FDC5233F1EFDC","740352F2412A9909880C23A559FCECEDA3BE2126FED62FC7660D628A06927F11"],"Flags":0,"Majorities":[{"Majority":{"Amendment":"1562511F573A19AE9BD103B5D6B9E01B3B46805AEC5D3C4805C902B514399146","CloseTime":535589001}}],"index":"7DB0788C020F02780A673DC74757F23823FA3014C1866E72CC4CD8B226CD6EF4"}"#;

        assert_eq!(expected, actual);
    }
}
