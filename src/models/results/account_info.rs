use serde::{Deserialize, Serialize};

use crate::models::ledger::AccountRoot;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo<'a> {
    pub account_data: AccountRoot<'a>,
}
