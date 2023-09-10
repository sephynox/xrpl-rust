//! Functions for encoding objects into the XRP Ledger's
//! canonical binary format and decoding them.

use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{asynch::transaction::PreparedTransaction, models::transactions::Transaction};

pub mod binary_wrappers;
pub mod exceptions;
pub(crate) mod test_cases;
pub mod types;
pub mod utils;

fn serialize_json<T>(
    prepared_transaction: PreparedTransaction<'_, T>,
    prefix: Option<&[u8]>,
    suffix: Option<&[u8]>,
    signing_only: bool,
) -> Value
where
    T: Transaction + Serialize + for<'de> Deserialize<'de> + Clone,
{
    let mut buffer = Vec::new();
    if let Some(pre) = prefix {
        buffer.extend_from_slice(pre);
    }

    todo!()
}
