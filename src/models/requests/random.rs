use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{Model, RequestMethod};

/// The random command provides a random number to be used
/// as a source of entropy for random number generation
/// by clients.
///
/// See Random:
/// `<https://xrpl.org/random.html#random>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Random<'a> {
    /// The unique request id.
    pub id: Option<&'a str>,
    /// The request method.
    #[serde(default = "RequestMethod::random")]
    pub command: RequestMethod,
}

impl Default for Random<'static> {
    fn default() -> Self {
        Random {
            id: None,
            command: RequestMethod::Random,
        }
    }
}

impl Model for Random<'static> {}
