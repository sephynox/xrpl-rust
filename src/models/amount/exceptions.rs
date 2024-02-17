use core::num::ParseFloatError;

use thiserror_no_std::Error;

#[derive(Debug, Error)]
pub enum XRPLAmountException {
    #[error("Unable to convert amount `value` into `Decimal`.")]
    ToDecimalError(#[from] rust_decimal::Error),
    #[error("Unable to convert amount `value` into `f64`.")]
    ToFloatError(#[from] ParseFloatError),
    #[error("{0:?}")]
    FromSerdeError(#[from] serde_json::Error),
}

#[cfg(feature = "std")]
impl alloc::error::Error for XRPLAmountException {}
