//! Helper util functions for the models module.
use crate::models::exceptions::JSONRPCException;
use alloc::string::String;
use serde::{Deserialize, Serialize};

/// JSONRPC Request
#[derive(Debug, Clone, Serialize)]
pub struct Request<'a, T> {
    pub method: &'a str,
    pub params: Option<T>,
    pub id: serde_json::Value,
    pub jsonrpc: Option<&'a str>,
}

/// JSONRPC Response
#[derive(Debug, Clone, Deserialize)]
pub struct Response<T> {
    pub id: serde_json::Value,
    pub result: Option<T>,
    pub error: Option<JSONRPCException>,
    pub jsonrpc: Option<String>,
}
