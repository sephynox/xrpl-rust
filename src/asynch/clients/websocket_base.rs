//! Traits used for all websocket clients.

use crate::models::Model;
use anyhow::Result;
use em_as_net::client::websocket::ReadResult;
use em_as_net::core::io::{AsyncRead, AsyncWrite};
use em_as_net::core::tcp::adapters::AdapterConnect;

use serde::Serialize;

// A client for interacting with the rippled WebSocket API.
pub trait WebsocketBase {
    fn is_open(&self) -> bool;
}

pub trait WebsocketOpen<'a, A, OpenWS>
where
    A: AdapterConnect<'a> + AsyncRead + AsyncWrite + Sized + Unpin,
{
    /// Connects to the host. To communicate with a host we need a TCP Socket to
    /// send bytes from the client to the host. This socket requires a preferred
    /// `adapter` like the `TcpAdapterTokio`. The adapter is the actual socket
    /// which must implement `AdapterConnect + AsyncRead + AsyncWrite + Sized + Unpin`
    async fn open(self, adapter: A) -> Result<OpenWS>;
}

pub trait WebsocketClose {
    /// Closes the websocket stream and disconnects from the host.
    async fn close(&mut self) -> Result<()>;
}

pub trait WebsocketIo {
    /// Writes the request to the stream.
    async fn write<R: Model + Serialize>(&mut self, request: &R) -> Result<()>;

    /// Read messages from the stream.
    async fn read(&mut self) -> Result<Option<ReadResult<'_>>>;
}
