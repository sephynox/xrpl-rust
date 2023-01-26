use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{Model, RequestMethod};

/// The ping command returns an acknowledgement, so that
/// clients can test the connection status and latency.
///
/// See Ping:
/// `<https://xrpl.org/ping.html#ping>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Ping<'a> {
    /// The unique request id.
    pub id: Option<&'a str>,
    /// The request method.
    #[serde(default = "RequestMethod::ping")]
    pub command: RequestMethod,
}

impl Default for Ping<'static> {
    fn default() -> Self {
        Ping {
            id: None,
            command: RequestMethod::Ping,
        }
    }
}

impl Model for Ping<'static> {}
