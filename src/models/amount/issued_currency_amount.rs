use crate::models::Model;
use alloc::borrow::Cow;
use core::convert::TryInto;
use core::str::FromStr;
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
    type Error = rust_decimal::Error;

    fn try_into(self) -> Result<Decimal, Self::Error> {
        Decimal::from_str(&self.value)
    }
}
