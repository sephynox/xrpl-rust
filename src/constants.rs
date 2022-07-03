//! Collection of public constants for XRPL.

use serde::{Deserialize, Serialize};
use strum_macros::Display;
use strum_macros::EnumIter;

/// Regular expression for determining ISO currency codes.
pub const ISO_CURRENCY_REGEX: &str = r"^[A-Z0-9]{3}$";
/// Regular expression for determining hex currency codes.
pub const HEX_CURRENCY_REGEX: &str = r"^[A-F0-9]{40}$";

/// Length of an account id.
pub const ACCOUNT_ID_LENGTH: usize = 20;

pub const TRANSACTION_HASH_PREFIX: &str = "54584E00";

/// Represents the supported cryptography algorithms.
#[derive(Debug, PartialEq, Clone, EnumIter, Display, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum CryptoAlgorithm {
    ED25519,
    SECP256K1,
}
