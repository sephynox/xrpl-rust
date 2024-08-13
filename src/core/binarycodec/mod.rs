//! Functions for encoding objects into the XRP Ledger's
//! canonical binary format and decoding them.

use core::fmt::Debug;

use alloc::{string::String, vec::Vec};
use anyhow::Result;
use hex::ToHex;
use serde::{de::DeserializeOwned, Serialize};
use strum::IntoEnumIterator;

use crate::{asynch::transaction::PreparedTransaction, models::transactions::Transaction};

use self::types::st_object::STObject;

pub mod binary_wrappers;
pub mod exceptions;
pub(crate) mod test_cases;
pub mod types;
pub mod utils;

const TRANSACTION_SIGNATURE_PREFIX: &'static str = "53545800";

pub fn encode_for_signing<'a, T, F>(
    prepared_transaction: &PreparedTransaction<'_, T>,
) -> Result<String>
where
    F: IntoEnumIterator + Serialize + Debug + PartialEq,
    T: Transaction<'a, F> + Serialize + DeserializeOwned + Clone,
{
    serialize_json(
        prepared_transaction,
        Some(
            serde_json::to_vec(TRANSACTION_SIGNATURE_PREFIX)
                .unwrap()
                .as_slice(),
        ),
        None,
        true,
    )
}

fn serialize_json<'a, T, F>(
    prepared_transaction: &PreparedTransaction<'_, T>,
    prefix: Option<&[u8]>,
    suffix: Option<&[u8]>,
    signing_only: bool,
) -> Result<String>
where
    F: IntoEnumIterator + Serialize + Debug + PartialEq,
    T: Transaction<'a, F> + Serialize + DeserializeOwned + Clone,
{
    let mut buffer = Vec::new();
    if let Some(p) = prefix {
        buffer.extend(p);
    }
    let json_value = serde_json::to_value(prepared_transaction).unwrap();
    let st_object = STObject::from_json_value(json_value, signing_only).unwrap();
    buffer.extend(st_object.buffer);
    if let Some(s) = suffix {
        buffer.extend(s);
    }

    let hex_string = buffer.encode_hex_upper::<String>();

    Ok(hex_string)
}
