use crate::models::requests::Request;
use alloc::string::String;
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
use serde_json::Value;
#[cfg(all(feature = "tungstenite", not(feature = "embedded-ws")))]
pub use tungstenite::*;

pub trait WebsocketClient<Status> {
    fn is_open(&self) -> bool {
        core::any::type_name::<Status>() == core::any::type_name::<WebsocketOpen>()
    }
}

#[derive(Serialize, Deserialize)]
pub struct XRPLResponse {
    pub id: Option<String>,
    pub result: Option<Value>,
    pub status: Option<String>,
    pub r#type: Option<String>,
    pub forwarded: Option<bool>,
    pub warnings: Option<Value>,
}

pub trait Client {
    async fn request(&mut self, req: impl Serialize) -> Result<XRPLResponse>;
}
