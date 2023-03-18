use crate::models::amount::ValueAsDecimal;
use crate::models::Model;
use alloc::borrow::Cow;
use core::str::FromStr;
use rust_decimal::Decimal;

pub type XRPAmount<'a> = Cow<'a, str>;

impl<'a> Model for XRPAmount<'a> {}

impl<'a> ValueAsDecimal for XRPAmount<'a> {
    fn as_decimal(&self) -> Result<Decimal, rust_decimal::Error> {
        Decimal::from_str(&*self)
    }
}
