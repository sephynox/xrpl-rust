use crate::models::amount::exceptions::XRPLAmountException;
use crate::models::Model;
use alloc::{
    borrow::Cow,
    string::{String, ToString},
};
use core::{
    convert::{TryFrom, TryInto},
    ops::{Add, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};
use core::{ops::AddAssign, str::FromStr};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents an amount of XRP in Drops.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Default)]
pub struct XRPAmount<'a>(pub Cow<'a, str>);

impl<'a> Model for XRPAmount<'a> {}

// implement Deserializing from Cow<str>, &str, String, Decimal, f64, u32, and Value
impl<'de, 'a> Deserialize<'de> for XRPAmount<'a> {
    fn deserialize<D>(deserializer: D) -> Result<XRPAmount<'a>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let amount_string = Value::deserialize(deserializer)?;
        Ok(XRPAmount::try_from(amount_string).map_err(serde::de::Error::custom)?)
    }
}

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

impl<'a> From<f64> for XRPAmount<'a> {
    fn from(value: f64) -> Self {
        Self(value.to_string().into())
    }
}

impl<'a> From<u32> for XRPAmount<'a> {
    fn from(value: u32) -> Self {
        Self(value.to_string().into())
    }
}

impl<'a> TryFrom<Value> for XRPAmount<'a> {
    type Error = XRPLAmountException;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let amount_string =
            serde_json::to_string(&value).map_err(XRPLAmountException::FromSerdeError)?;
        let amount_string = amount_string.clone().replace("\"", "");
        Ok(Self(amount_string.into()))
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

impl<'a> PartialOrd for XRPAmount<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl<'a> Ord for XRPAmount<'a> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<'a> Add for XRPAmount<'a> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let self_decimal: Decimal = self.try_into().unwrap();
        let other_decimal: Decimal = other.try_into().unwrap();
        let result_decimal = self_decimal + other_decimal;
        result_decimal.into()
    }
}

impl<'a> AddAssign for XRPAmount<'a> {
    fn add_assign(&mut self, other: Self) {
        let self_decimal: Decimal = self.clone().try_into().unwrap();
        let other_decimal: Decimal = other.try_into().unwrap();
        let result_decimal = self_decimal + other_decimal;
        *self = result_decimal.into();
    }
}

impl<'a> Sub for XRPAmount<'a> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let self_decimal: Decimal = self.try_into().unwrap();
        let other_decimal: Decimal = other.try_into().unwrap();
        let result_decimal = self_decimal - other_decimal;
        result_decimal.into()
    }
}

impl<'a> SubAssign for XRPAmount<'a> {
    fn sub_assign(&mut self, other: Self) {
        let self_decimal: Decimal = self.clone().try_into().unwrap();
        let other_decimal: Decimal = other.try_into().unwrap();
        let result_decimal = self_decimal - other_decimal;
        *self = result_decimal.into();
    }
}

impl<'a> Mul for XRPAmount<'a> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let self_decimal: Decimal = self.try_into().unwrap();
        let other_decimal: Decimal = other.try_into().unwrap();
        let result_decimal = self_decimal * other_decimal;
        result_decimal.into()
    }
}

impl<'a> Mul<u8> for XRPAmount<'a> {
    type Output = Self;

    fn mul(self, other: u8) -> Self {
        let self_decimal: Decimal = self.try_into().unwrap();
        let other_decimal: Decimal = other.try_into().unwrap();
        let result_decimal = self_decimal * other_decimal;
        result_decimal.into()
    }
}

impl<'a> MulAssign for XRPAmount<'a> {
    fn mul_assign(&mut self, other: Self) {
        let self_decimal: Decimal = self.clone().try_into().unwrap();
        let other_decimal: Decimal = other.try_into().unwrap();
        let result_decimal = self_decimal * other_decimal;
        *self = result_decimal.into();
    }
}

impl<'a> Div for XRPAmount<'a> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let self_decimal: Decimal = self.try_into().unwrap();
        let other_decimal: Decimal = other.try_into().unwrap();
        let result_decimal = self_decimal / other_decimal;
        result_decimal.into()
    }
}

impl<'a> DivAssign for XRPAmount<'a> {
    fn div_assign(&mut self, other: Self) {
        let self_decimal: Decimal = self.clone().try_into().unwrap();
        let other_decimal: Decimal = other.try_into().unwrap();
        let result_decimal = self_decimal / other_decimal;
        *self = result_decimal.into();
    }
}

impl XRPAmount<'_> {
    pub fn ceil(&self) -> Self {
        let decimal: Decimal = self.clone().try_into().unwrap();
        let result_decimal = decimal.ceil();
        result_decimal.into()
    }

    pub fn floor(&self) -> Self {
        let decimal: Decimal = self.clone().try_into().unwrap();
        let result_decimal = decimal.floor();
        result_decimal.into()
    }
}
