use crate::models::amount::exceptions::XRPLAmountException;
use crate::models::Model;
use alloc::{
    borrow::Cow,
    string::{String, ToString},
};
use core::convert::TryInto;
use core::str::FromStr;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents an amount of XRP in Drops.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct XRPAmount<'a>(pub Cow<'a, str>);

impl<'a> Model for XRPAmount<'a> {}

impl<'a> From<Cow<'a, str>> for XRPAmount<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a str> for XRPAmount<'a> {
    fn from(value: &'a str) -> Self {
        Self(value.into())
    }
}

impl<'a> From<String> for XRPAmount<'a> {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl<'a> From<Decimal> for XRPAmount<'a> {
    fn from(value: Decimal) -> Self {
        Self(value.to_string().into())
    }
}

impl<'a> TryInto<f64> for XRPAmount<'a> {
    type Error = XRPLAmountException;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self.0.parse::<f64>() {
            Ok(f64_value) => Ok(f64_value),
            Err(parse_error) => Err(XRPLAmountException::ToFloatError(parse_error)),
        }
    }
}

impl<'a> TryInto<Decimal> for XRPAmount<'a> {
    type Error = XRPLAmountException;

    fn try_into(self) -> Result<Decimal, Self::Error> {
        match Decimal::from_str(&self.0) {
            Ok(decimal) => Ok(decimal),
            Err(decimal_error) => Err(XRPLAmountException::ToDecimalError(decimal_error)),
        }
    }
}
