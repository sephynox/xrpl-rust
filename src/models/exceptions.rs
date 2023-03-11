//! General XRPL Model Exception.

use crate::models::requests::XrplRequestException;
use crate::models::transactions::XrplTransactionException;
use alloc::string::String;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, PartialEq, Display)]
#[non_exhaustive]
pub enum XrplModelException<'a> {
    InvalidICCannotBeXRP,
    XrplTransactionError(XrplTransactionException<'a>),
    XrplRequestError(XrplRequestException<'a>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct JSONRPCException {
    code: i32,
    message: String,
}

#[cfg(feature = "std")]
impl<'a> alloc::error::Error for XrplModelException<'a> {}
