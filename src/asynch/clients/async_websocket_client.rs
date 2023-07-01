use crate::asynch::clients::exceptions::XRPLWebsocketException;
use crate::models::Model;
use crate::Err;

// ! EXPORTS
pub use crate::asynch::clients::websocket_base::{
    WebsocketBase, WebsocketClose, WebsocketIo, WebsocketOpen,
};
pub use em_as_net::client::websocket::ReadResult;
pub use em_as_net::core::io::{AsyncRead, AsyncWrite};
// AsyncWebSocketClient
#[cfg(feature = "std")]
pub type AsyncWebsocketClient<'a, T, Rng, Status = Closed> =
    if_std::AsyncWebsocketClient<'a, T, Rng, Status>;
pub use em_as_net::core::tcp::TcpSocket;
// TCP Adapters
pub use em_as_net::core::tcp::adapters::TcpAdapterTokio;
// Build your own TCP Socket
pub use em_as_net::core::tcp::TcpConnect;
// Build your own TCP Adapter
pub use em_as_net::core::tcp::adapters::AdapterConnect;
// Websocket statuses
pub struct Open;
pub struct Closed;

use anyhow::Result;
use em_as_net::client::websocket::{WebsocketClientIo, WebsocketSendMessageType};
use rand::RngCore;
use serde::Serialize;

#[cfg(feature = "std")]
mod if_std {
    use super::{AsyncRead, AsyncWrite, Closed, Open, WebsocketBase, WebsocketOpen};

    use alloc::borrow::Cow;
    use core::marker::PhantomData;

    use anyhow::Result;
    use em_as_net::client::websocket::{WebsocketClient, WebsocketClientConnect};
    use em_as_net::core::tcp::adapters::AdapterConnect;
    use em_as_net::core::tcp::{TcpConnect, TcpSocket};
    use rand::rngs::ThreadRng;
    use rand::{thread_rng, RngCore};

    /// An async client for interacting with the rippled WebSocket API.
    pub struct AsyncWebsocketClient<'a, T, Rng, Status = Closed>
    where
        T: TcpConnect<'a> + AsyncRead + AsyncWrite,
        Rng: RngCore,
    {
        pub uri: Cow<'a, str>,
        pub(crate) inner: WebsocketClient<'a, T, Rng>,
        pub(crate) status: PhantomData<Status>,
    }

    impl<'a, T, Rng, Status> AsyncWebsocketClient<'a, T, Rng, Status>
    where
        T: TcpConnect<'a> + AsyncRead + AsyncWrite,
        Rng: RngCore,
    {
        pub fn new(uri: Cow<'a, str>, buffer: &'a mut [u8]) -> Self {
            Self {
                uri: uri.clone(),
                inner: WebsocketClient::new(uri.clone(), buffer),
                status: PhantomData::default(),
            }
        }
    }

    impl<'a, T, Rng, Status> WebsocketBase for AsyncWebsocketClient<'a, T, Rng, Status>
    where
        T: TcpConnect<'a> + AsyncRead + AsyncWrite,
        Rng: RngCore,
    {
        fn is_open(&self) -> bool {
            self.status == PhantomData::<Status>
        }
    }

    impl<'a, A> WebsocketOpen<'a, A, AsyncWebsocketClient<'a, TcpSocket<A>, ThreadRng, Open>>
        for AsyncWebsocketClient<'a, TcpSocket<A>, ThreadRng, Closed>
    where
        A: AdapterConnect<'a> + AsyncRead + AsyncWrite + Sized + Unpin,
    {
        async fn open(
            mut self,
            adapter: A,
        ) -> Result<AsyncWebsocketClient<'a, TcpSocket<A>, ThreadRng, Open>> {
            let tcp_socket = TcpSocket::new(adapter);
            let rng = thread_rng();
            self.inner
                .connect(tcp_socket, None, rng)
                .await
                .expect("TODO: panic message");

            Ok(AsyncWebsocketClient {
                uri: self.uri,
                inner: self.inner,
                status: PhantomData::<Open>,
            })
        }
    }
}

impl<'a, T, Rng> WebsocketIo for AsyncWebsocketClient<'a, T, Rng, Open>
where
    T: TcpConnect<'a> + AsyncRead + AsyncWrite + Unpin,
    Rng: RngCore,
{
    async fn write<R: Model + Serialize>(&mut self, request: &R) -> Result<()> {
        let request_json = match serde_json::to_string(&request) {
            Ok(as_string) => as_string,
            Err(_) => return Err!(XRPLWebsocketException::RequestSerializationError),
        };
        self.inner
            .write(request_json.into(), Some(WebsocketSendMessageType::Text))
            .await
    }

    async fn read(&mut self) -> Result<Option<ReadResult<'_>>> {
        match self.inner.read().await {
            None => Ok(None),
            Some(Ok(read_result)) => Ok(Some(read_result)),
            Some(Err(read_error)) => Err(read_error),
        }
    }
}

impl<'a, T, Rng> WebsocketClose for AsyncWebsocketClient<'a, T, Rng, Open>
where
    T: TcpConnect<'a> + AsyncRead + AsyncWrite + Unpin,
    Rng: RngCore,
{
    async fn close(&mut self) -> Result<()> {
        self.inner.close().await
    }
}
