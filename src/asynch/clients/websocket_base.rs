use crate::asynch::clients::client::Client;
use crate::models::Model;
use anyhow::Result;
use em_as_net::client::websocket::ReadResult;
use serde::Serialize;

// A client for interacting with the rippled WebSocket API.
pub trait WebsocketBase<'a>: Client<'a> {
    fn is_open(&self) -> bool;

    async fn do_open(&self) -> Result<()>;

    async fn do_close(&self) -> Result<()>;

    async fn do_write<T: Model + Serialize>(&self, request: T) -> Result<()>;

    async fn do_read(&'a mut self) -> Result<Option<ReadResult<'a>>>;

    async fn do_request_impl<T: Model + Serialize, R>(&mut self, request: T) -> Result<R>;
}
