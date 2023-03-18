pub mod issued_currency_amount;
pub mod xrp_amount;

pub use issued_currency_amount::*;
use rust_decimal::{Decimal, Error};
pub use xrp_amount::*;

use crate::models::Model;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

pub trait ValueAsDecimal {
    fn as_decimal(&self) -> Result<Decimal, Error>;
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Display)]
#[serde(untagged)]
pub enum Amount<'a> {
    IssuedCurrencyAmount(IssuedCurrencyAmount<'a>),
    XRPAmount(XRPAmount<'a>),
}

impl<'a> ValueAsDecimal for Amount<'a> {
    fn as_decimal(&self) -> Result<Decimal, Error> {
        match self {
            Amount::IssuedCurrencyAmount(amount) => amount.as_decimal(),
            Amount::XRPAmount(amount) => amount.as_decimal(),
        }
    }
}

impl<'a> Model for Amount<'a> {}

impl<'a> Default for Amount<'a> {
    fn default() -> Self {
        Self::XRPAmount("0".into())
    }
}

impl<'a> Amount<'a> {
    pub fn is_xrp(&self) -> bool {
        match self {
            Amount::IssuedCurrencyAmount(_) => false,
            Amount::XRPAmount(_) => true,
        }
    }

    pub fn is_issued_currency(&self) -> bool {
        !self.is_xrp()
    }
}

impl<'a> From<IssuedCurrencyAmount<'a>> for Amount<'a> {
    fn from(value: IssuedCurrencyAmount<'a>) -> Self {
        Self::IssuedCurrencyAmount(value)
    }
}

impl<'a> From<XRPAmount<'a>> for Amount<'a> {
    fn from(value: XRPAmount<'a>) -> Self {
        Self::XRPAmount(value)
    }
}
