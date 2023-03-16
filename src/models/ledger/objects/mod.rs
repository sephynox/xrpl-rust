pub mod account_root;
pub mod amendments;
pub mod amm;
pub mod check;

pub use account_root::*;
pub use amendments::*;
pub use amm::*;
pub use check::*;

use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Clone, Serialize, Deserialize, Display, PartialEq, Eq)]
pub enum LedgerEntryType {
    AccountRoot = 0x0061,
    Amendments = 0x0066,
    AMM = 0x0079,
    Check = 0x0043,
}
