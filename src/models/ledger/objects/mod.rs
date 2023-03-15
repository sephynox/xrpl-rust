pub mod account_root;
pub mod amendments;
pub mod amm;

pub use account_root::*;
pub use amendments::*;

use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Clone, Serialize, Deserialize, Display, PartialEq, Eq)]
pub enum LedgerEntryType {
    AccountRoot = 0x0061,
    Amendments = 0x0066,
    AMM,
}
