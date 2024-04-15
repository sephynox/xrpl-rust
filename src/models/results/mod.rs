pub mod account_info;
pub mod fee;
pub mod ledger;
pub mod server_state;

pub use account_info::*;
pub use fee::*;
pub use ledger::*;
pub use server_state::*;

use alloc::{borrow::Cow, vec::Vec};
use anyhow::Result;
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Err;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
    Success,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResponseType {
    Response,
    LedgerClosed,
    Transaction,
}

pub trait XRPLResponseFromStream<T: for<'de> Deserialize<'de>>:
    StreamExt<Item = Result<Value>>
{
    async fn next_xrpl_response(&mut self) -> Option<Result<XRPLResponse<'_, T>>>
    where
        Self: Unpin;

    async fn try_next_xrpl_response(&mut self) -> Result<Option<XRPLResponse<'_, T>>>
    where
        Self: Unpin,
    {
        match self.next_xrpl_response().await {
            Some(response) => response.map(Some),
            None => Ok(None),
        }
    }
}

impl<S, T> XRPLResponseFromStream<T> for S
where
    S: Stream<Item = Result<Value>> + StreamExt<Item = Result<Value>> + Unpin,
    T: for<'de> Deserialize<'de>,
{
    async fn next_xrpl_response(&mut self) -> Option<Result<XRPLResponse<'_, T>>>
    where
        Self: StreamExt<Item = Result<Value>>,
    {
        let item = self.next().await;
        match item {
            Some(Ok(message)) => match serde_json::from_value(message) {
                Ok(response) => Some(Ok(response)),
                Err(error) => Some(Err!(error)),
            },
            Some(Err(error)) => Some(Err!(error)),
            None => None,
        }
    }
}

/// A response from a XRPL node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XRPLResponse<'a, T> {
    pub id: Option<Cow<'a, str>>,
    pub result: T,
    pub status: Option<ResponseStatus>,
    pub r#type: Option<ResponseType>,
    pub forwarded: Option<bool>,
    pub warnings: Option<Vec<XRPLWarning<'a>>>,
    pub warning: Option<Cow<'a, str>>,
}

impl<T> XRPLResponse<'_, T> {
    pub fn is_successful(&self) -> bool {
        match self.status {
            Some(ResponseStatus::Success) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XRPLErrorResponse<'a, T> {
    pub id: Cow<'a, str>,
    pub error: Option<Cow<'a, str>>,
    pub error_code: Option<i32>,
    pub error_message: Option<Cow<'a, str>>,
    pub request: Option<T>,
    pub status: Option<ResponseStatus>,
    pub r#type: Option<ResponseType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XRPLWarning<'a> {
    pub id: Cow<'a, str>,
    pub message: Cow<'a, str>,
    pub forwarded: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyResult;
