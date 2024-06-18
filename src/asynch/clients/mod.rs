mod async_client;
mod client;
mod json_rpc;
mod websocket;

use alloc::borrow::Cow;
use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex};
use serde::{Deserialize, Serialize};

pub use async_client::*;
pub use json_rpc::*;
pub use websocket::*;

pub type MultiExecutorMutex = CriticalSectionRawMutex;
pub type SingleExecutorMutex = NoopRawMutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonFields<'a> {
    pub build_version: Option<Cow<'a, str>>,
    pub network_id: Option<u32>,
}
