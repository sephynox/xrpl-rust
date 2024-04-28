use alloc::borrow::Cow;
use anyhow::Result;

pub mod exceptions;

pub struct WebsocketOpen;
pub struct WebsocketClosed;

#[cfg(all(feature = "embedded-ws", not(feature = "tungstenite")))]
mod embedded_websocket;
#[cfg(all(feature = "tungstenite", not(feature = "embedded-ws")))]
mod tungstenite;

#[cfg(all(feature = "embedded-ws", not(feature = "tungstenite")))]
pub use embedded_websocket::*;
use serde::{Deserialize, Serialize};
#[cfg(all(feature = "tungstenite", not(feature = "embedded-ws")))]
pub use tungstenite::*;

use crate::models::results::XRPLResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonFields<'a> {
    pub build_version: Option<Cow<'a, str>>,
    pub network_id: Option<u32>,
}

pub trait WebsocketClient<Status> {
    fn is_open(&self) -> bool {
        core::any::type_name::<Status>() == core::any::type_name::<WebsocketOpen>()
    }
}

pub trait Client<'a> {
    async fn request<T>(&self, req: impl Serialize) -> Result<XRPLResponse<'a, T>>
    where
        T: for<'de> Deserialize<'de> + Clone;

    async fn get_common_fields(&self) -> Result<CommonFields<'a>>;
}
