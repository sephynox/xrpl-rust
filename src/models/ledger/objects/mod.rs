pub mod account_root;
pub mod amendments;
pub mod amm;
pub mod check;
pub mod deposit_preauth;
pub mod directory_node;
pub mod escrow;
pub mod fee_settings;

pub use account_root::*;
pub use amendments::*;
pub use amm::*;
pub use check::*;
pub use deposit_preauth::*;
pub use directory_node::*;
pub use escrow::*;
pub use fee_settings::*;

use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Clone, Serialize, Deserialize, Display, PartialEq, Eq)]
pub enum LedgerEntryType {
    AccountRoot = 0x0061,
    Amendments = 0x0066,
    AMM = 0x0079,
    Check = 0x0043,
    DepositPreauth = 0x0070,
    DirectoryNode = 0x0064,
    Escrow = 0x0075,
    FeeSettings = 0x0073,
}
