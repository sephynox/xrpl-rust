//! Exception for invalid XRP Ledger type data.

use crate::core::addresscodec::exceptions::XRPLAddressCodecException;
use crate::core::binarycodec::exceptions::XRPLBinaryCodecException;
use crate::utils::exceptions::ISOCodeException;
use crate::utils::exceptions::JSONParseException;
use crate::utils::exceptions::XRPRangeException;
use strum_macros::Display;
use thiserror_no_std::Error;

#[derive(Debug, Clone, PartialEq, Display)]
#[non_exhaustive]
pub enum XRPLTypeException {
    InvalidNoneValue,
    FromHexError,
    XRPLBinaryCodecError(XRPLBinaryCodecException),
    XRPLHashError(XRPLHashException),
    XRPLRangeError(XRPRangeException),
    DecimalError(rust_decimal::Error),
    JSONParseError(JSONParseException),
    UnknownXRPLType,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum XRPLSerializeArrayException {
    #[error("Expected `Value` to be an array.")]
    ExpectedArray,
    #[error("Expected `Value` to be an array of objects.")]
    ExpectedObjectArray,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum XRPLSerializeMapException<'a> {
    #[error("Expected `Value` to be an object.")]
    ExpectedObject,
    #[error("Field `{field}` is not allowed to have an associated tag.")]
    DisallowedTag { field: &'a str },
    #[error("Cannot have mismatched Account X-Address and SourceTag")]
    AccountMismatchingTags,
    #[error("Cannot have mismatched Destination X-Address and DestinationTag")]
    DestinationMismatchingTags,
    #[error("Unknown transaction type: {0}")]
    UnknownTransactionType(&'a str),
    #[error("Unknown transaction result: {0}")]
    UnknownTransactionResult(&'a str),
    #[error("Unknown ledger entry type: {0}")]
    UnknownLedgerEntryType(&'a str),
}

#[derive(Debug, Clone, PartialEq, Display)]
#[non_exhaustive]
pub enum XRPLHashException {
    InvalidHashLength { expected: usize, found: usize },
    FromHexError,
    ISOCodeError(ISOCodeException),
    XRPLBinaryCodecError(XRPLBinaryCodecException),
    XRPLAddressCodecError(XRPLAddressCodecException),
    SerdeJsonError(serde_json::error::Category),
}

#[derive(Debug, Clone, PartialEq, Display)]
#[non_exhaustive]
pub enum XRPLVectorException {
    InvalidVector256Bytes,
    XRPLBinaryCodecError(XRPLBinaryCodecException),
    XRPLHashError(XRPLHashException),
}

impl From<XRPLHashException> for XRPLTypeException {
    fn from(err: XRPLHashException) -> Self {
        XRPLTypeException::XRPLHashError(err)
    }
}

impl From<XRPRangeException> for XRPLTypeException {
    fn from(err: XRPRangeException) -> Self {
        XRPLTypeException::XRPLRangeError(err)
    }
}

impl From<XRPLBinaryCodecException> for XRPLTypeException {
    fn from(err: XRPLBinaryCodecException) -> Self {
        XRPLTypeException::XRPLBinaryCodecError(err)
    }
}

impl From<JSONParseException> for XRPLTypeException {
    fn from(err: JSONParseException) -> Self {
        XRPLTypeException::JSONParseError(err)
    }
}

impl From<rust_decimal::Error> for XRPLTypeException {
    fn from(err: rust_decimal::Error) -> Self {
        XRPLTypeException::DecimalError(err)
    }
}

impl From<hex::FromHexError> for XRPLTypeException {
    fn from(_: hex::FromHexError) -> Self {
        XRPLTypeException::FromHexError
    }
}

impl From<ISOCodeException> for XRPLHashException {
    fn from(err: ISOCodeException) -> Self {
        XRPLHashException::ISOCodeError(err)
    }
}

impl From<XRPLBinaryCodecException> for XRPLHashException {
    fn from(err: XRPLBinaryCodecException) -> Self {
        XRPLHashException::XRPLBinaryCodecError(err)
    }
}

impl From<XRPLAddressCodecException> for XRPLHashException {
    fn from(err: XRPLAddressCodecException) -> Self {
        XRPLHashException::XRPLAddressCodecError(err)
    }
}

impl From<serde_json::Error> for XRPLHashException {
    fn from(err: serde_json::Error) -> Self {
        XRPLHashException::SerdeJsonError(err.classify())
    }
}

impl From<hex::FromHexError> for XRPLHashException {
    fn from(_: hex::FromHexError) -> Self {
        XRPLHashException::FromHexError
    }
}

impl From<XRPLHashException> for XRPLVectorException {
    fn from(err: XRPLHashException) -> Self {
        XRPLVectorException::XRPLHashError(err)
    }
}

#[cfg(feature = "std")]
impl alloc::error::Error for XRPLTypeException {}

#[cfg(feature = "std")]
impl alloc::error::Error for XRPLHashException {}

#[cfg(feature = "std")]
impl alloc::error::Error for XRPLVectorException {}
