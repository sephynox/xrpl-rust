mod account_info;
mod exceptions;
mod fee;
mod ledger;
mod server_state;

pub use account_info::*;
pub use exceptions::*;
pub use fee::*;
pub use ledger::*;
pub use server_state::*;

use alloc::{borrow::Cow, format, string::ToString, vec::Vec};
use anyhow::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::Err;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
    Success,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ResponseType {
    Response,
    LedgerClosed,
    Transaction,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct XRPLResponse<'a, Res, Req> {
    pub id: Option<Cow<'a, str>>,
    pub error: Option<Cow<'a, str>>,
    pub error_code: Option<i32>,
    pub error_message: Option<Cow<'a, str>>,
    pub forwarded: Option<bool>,
    pub request: Option<Req>,
    pub result: Option<Res>,
    pub status: Option<ResponseStatus>,
    pub r#type: Option<ResponseType>,
    pub warning: Option<Cow<'a, str>>,
    pub warnings: Option<Vec<XRPLWarning<'a>>>,
}

impl<'a, 'de, Res, Req> Deserialize<'de> for XRPLResponse<'a, Res, Req>
where
    Res: DeserializeOwned,
    Req: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // TODO: add validation for fields that can not coexist in the same response
        let mut map = serde_json::Map::deserialize(deserializer)?;
        if map.is_empty() {
            return Err(serde::de::Error::custom("Empty response"));
        }
        let request = match map.remove("request") {
            Some(request) => match serde_json::from_value(request) {
                Ok(request) => request,
                Err(error) => return Err(serde::de::Error::custom(error.to_string())),
            },
            None => None,
        };
        let result = match map.remove("result") {
            Some(result) => match serde_json::from_value(result) {
                Ok(result) => result,
                Err(error) => return Err(serde::de::Error::custom(error.to_string())),
            },
            None => None,
        };
        let status = match map.remove("status") {
            Some(status) => match serde_json::from_value(status) {
                Ok(status) => status,
                Err(error) => return Err(serde::de::Error::custom(error.to_string())),
            },
            None => None,
        };
        let r#type = match map.remove("type") {
            Some(r#type) => match serde_json::from_value(r#type) {
                Ok(r#type) => r#type,
                Err(error) => return Err(serde::de::Error::custom(error.to_string())),
            },
            None => None,
        };
        Ok(XRPLResponse {
            id: map.remove("id").map(|item| match item.as_str() {
                Some(item_str) => Cow::Owned(item_str.to_string()),
                None => Cow::Borrowed(""),
            }),
            error: map.remove("error").map(|item| match item.as_str() {
                Some(item_str) => Cow::Owned(item_str.to_string()),
                None => Cow::Borrowed(""),
            }),
            error_code: map
                .remove("error_code")
                .and_then(|v| v.as_i64())
                .map(|v| v as i32),
            error_message: map.remove("error_message").map(|item| match item.as_str() {
                Some(item_str) => Cow::Owned(item_str.to_string()),
                None => Cow::Borrowed(""),
            }),
            forwarded: map.remove("forwarded").and_then(|v| v.as_bool()),
            request,
            result,
            status,
            r#type,
            warning: map.remove("warning").map(|item| match item.as_str() {
                Some(item_str) => Cow::Owned(item_str.to_string()),
                None => Cow::Borrowed(""),
            }),
            warnings: map
                .remove("warnings")
                .and_then(|v| serde_json::from_value(v).ok()),
        })
    }
}

impl<Res, Req> XRPLResponse<'_, Res, Req> {
    pub fn is_success(&self) -> bool {
        self.status == Some(ResponseStatus::Success)
    }

    pub fn try_into_result(self) -> Result<Res> {
        match self.result {
            Some(result) => Ok(result),
            None => {
                if let Some(error) = self.error {
                    Err!(XRPLResultException::ResponseError(format!(
                        "{}: {}",
                        error,
                        self.error_message.unwrap_or_default()
                    )))
                } else {
                    Err!(XRPLResultException::ExpectedResultOrError)
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct XRPLWarning<'a> {
    pub id: Cow<'a, str>,
    pub message: Cow<'a, str>,
    pub forwarded: Option<bool>,
}
