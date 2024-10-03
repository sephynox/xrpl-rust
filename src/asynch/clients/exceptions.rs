use core::fmt::Debug;

use thiserror_no_std::Error;
use url::ParseError;

use crate::asynch::XRPLFaucetException;

use super::{XRPLJsonRpcException, XRPLWebSocketException};

pub type XRPLClientResult<T> = Result<T, XRPLClientException>;

#[derive(Debug, Error)]
pub enum XRPLClientException {
    #[error("{0:?}")]
    XRPLJsonRpcException(#[from] XRPLJsonRpcException),
    #[error("{0:?}")]
    XRPLWebSocketException(#[from] XRPLWebSocketException),
    #[error("{0:?}")]
    XRPLFaucetException(#[from] XRPLFaucetException),
    #[error("{0:?}")]
    UrlParseError(#[from] ParseError),
    #[error("{0:?}")]
    SerdeError(#[from] serde_json::Error),
}
