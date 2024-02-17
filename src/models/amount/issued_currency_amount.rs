use crate::models::amount::exceptions::XRPLAmountException;
use crate::models::Model;
use alloc::borrow::Cow;
use alloc::string::ToString;
use core::ops::{AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use core::str::FromStr;
use core::{convert::TryInto, ops::Add};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct IssuedCurrencyAmount<'a> {
    pub currency: Cow<'a, str>,
    pub issuer: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

impl<'a> Model for IssuedCurrencyAmount<'a> {}

impl<'a> IssuedCurrencyAmount<'a> {
    pub fn new(currency: Cow<'a, str>, issuer: Cow<'a, str>, value: Cow<'a, str>) -> Self {
        Self {
            currency,
            issuer,
            value,
        }
    }
}

impl<'a> TryInto<Decimal> for IssuedCurrencyAmount<'a> {
    type Error = XRPLAmountException;

    fn try_into(self) -> Result<Decimal, Self::Error> {
        match Decimal::from_str(&self.value) {
            Ok(decimal) => Ok(decimal),
            Err(decimal_error) => Err(XRPLAmountException::ToDecimalError(decimal_error)),
        }
    }
}

impl<'a> PartialOrd for IssuedCurrencyAmount<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<'a> Ord for IssuedCurrencyAmount<'a> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl<'a> Add for IssuedCurrencyAmount<'a> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let value =
            Decimal::from_str(&self.value).unwrap() + Decimal::from_str(&other.value).unwrap();
        Self {
            currency: self.currency,
            issuer: self.issuer,
            value: value.to_string().into(),
        }
    }
}

impl<'a> AddAssign for IssuedCurrencyAmount<'a> {
    fn add_assign(&mut self, other: Self) {
        let value =
            Decimal::from_str(&self.value).unwrap() + Decimal::from_str(&other.value).unwrap();
        self.value = value.to_string().into();
    }
}

impl<'a> Sub for IssuedCurrencyAmount<'a> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let value =
            Decimal::from_str(&self.value).unwrap() - Decimal::from_str(&other.value).unwrap();
        Self {
            currency: self.currency,
            issuer: self.issuer,
            value: value.to_string().into(),
        }
    }
}

impl<'a> SubAssign for IssuedCurrencyAmount<'a> {
    fn sub_assign(&mut self, other: Self) {
        let value =
            Decimal::from_str(&self.value).unwrap() - Decimal::from_str(&other.value).unwrap();
        self.value = value.to_string().into();
    }
}

impl<'a> Mul for IssuedCurrencyAmount<'a> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let value =
            Decimal::from_str(&self.value).unwrap() * Decimal::from_str(&other.value).unwrap();
        Self {
            currency: self.currency,
            issuer: self.issuer,
            value: value.to_string().into(),
        }
    }
}

impl<'a> MulAssign for IssuedCurrencyAmount<'a> {
    fn mul_assign(&mut self, other: Self) {
        let value =
            Decimal::from_str(&self.value).unwrap() * Decimal::from_str(&other.value).unwrap();
        self.value = value.to_string().into();
    }
}

impl<'a> Div for IssuedCurrencyAmount<'a> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let value =
            Decimal::from_str(&self.value).unwrap() / Decimal::from_str(&other.value).unwrap();
        Self {
            currency: self.currency,
            issuer: self.issuer,
            value: value.to_string().into(),
        }
    }
}

impl<'a> DivAssign for IssuedCurrencyAmount<'a> {
    fn div_assign(&mut self, other: Self) {
        let value =
            Decimal::from_str(&self.value).unwrap() / Decimal::from_str(&other.value).unwrap();
        self.value = value.to_string().into();
    }
}
