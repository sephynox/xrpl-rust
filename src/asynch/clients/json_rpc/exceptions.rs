use core::result::Result;

use thiserror_no_std::Error;

pub type XRPLJsonRpcResult<T> = Result<T, XRPLJsonRpcException>;

#[derive(Debug, Error)]
pub enum XRPLJsonRpcException {
    #[error("Reqwless error")]
    ReqwlessError,
    #[cfg(feature = "std")]
    #[error("Request error: {0:?}")]
    ReqwestError(reqwest::Error),
    #[error("0:?")]
    SerdeError(#[from] serde_json::Error),
    #[error("Expected JSON object")]
    ExpectedJsonObject,
}
