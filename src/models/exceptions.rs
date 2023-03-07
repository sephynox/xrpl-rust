//! General XRPL Model Exception.

use alloc::string::String;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, PartialEq, Display)]
#[non_exhaustive]
pub enum XRPLModelException {
    InvalidICCannotBeXRP,
    XRPLTransactionError(XRPLTransactionException),
    XRPLRequestError(XRPLRequestException),
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum XRPLRequestException {
    ChannelAuthorizeError(ChannelAuthorizeException),
    SignAndSubmitError(SignAndSubmitException),
    SignForError(SignForException),
    SignError(SignException),
    LedgerEntryError(LedgerEntryException),
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum ChannelAuthorizeException {
    InvalidMustSetExactlyOneOf { fields: String },
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum LedgerEntryException {
    InvalidMustSetExactlyOneOf { fields: String },
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum SignAndSubmitException {
    InvalidMustSetExactlyOneOf { fields: String },
    InvalidMustOmitKeyTypeIfSecretProvided,
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum SignForException {
    InvalidMustSetExactlyOneOf { fields: String },
    InvalidMustOmitKeyTypeIfSecretProvided,
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum SignException {
    InvalidMustSetExactlyOneOf { fields: String },
    InvalidMustOmitKeyTypeIfSecretProvided,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct JSONRPCException {
    code: i32,
    message: String,
}

#[cfg(feature = "std")]
impl alloc::error::Error for XRPLModelException {}
