use super::{
    exceptions::XRPLWebsocketException, Client, CommonFields, WebsocketClient, WebsocketClosed,
    WebsocketOpen,
};
use crate::models::requests;
use crate::models::results::{self, XRPLResponse};
use crate::Err;

use alloc::sync::Arc;
use anyhow::Result;
use core::marker::PhantomData;
use core::{pin::Pin, task::Poll};
use futures::lock::Mutex;
use futures::{FutureExt, Sink, Stream, StreamExt};
use futures_util::SinkExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async as tungstenite_connect_async, MaybeTlsStream as TungsteniteMaybeTlsStream,
    WebSocketStream as TungsteniteWebsocketStream,
};
use url::Url;

pub use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;

pub(crate) enum ContextWaker {
    Read,
    Write,
}

pub type AsyncWebsocketConnection =
    Arc<Mutex<TungsteniteWebsocketStream<TungsteniteMaybeTlsStream<TcpStream>>>>;

pub struct AsyncWebsocketClient<Status = WebsocketClosed> {
    inner: AsyncWebsocketConnection,
    status: PhantomData<Status>,
}

impl<'a, Status> WebsocketClient<Status> for AsyncWebsocketClient<Status> {}

impl<'a, I> Sink<I> for AsyncWebsocketClient<WebsocketOpen>
where
    I: serde::Serialize,
    Self: Unpin,
{
    type Error = anyhow::Error;

    fn poll_ready(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<()>> {
        let mut guard = futures::ready!(self.inner.lock().poll_unpin(cx));
        match Pin::new(&mut *guard).poll_ready(cx) {
            Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
            Poll::Ready(Err(error)) => Poll::Ready(Err!(error)),
            Poll::Pending => Poll::Pending,
        }
    }

    fn start_send(self: core::pin::Pin<&mut Self>, item: I) -> Result<()> {
        match serde_json::to_string(&item) {
            Ok(json) => {
                // cannot use ready! macro here because of the return type
                let _ = self.inner.lock().then(|mut guard| async move {
                    match Pin::new(&mut *guard)
                        .send(TungsteniteMessage::Text(json))
                        .await
                    {
                        Ok(_) => Ok(()),
                        Err(error) => Err!(error),
                    }
                });
                Ok(())
            }
            Err(error) => Err!(error),
        }
    }

    fn poll_flush(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<()>> {
        let mut guard = futures::ready!(self.inner.lock().poll_unpin(cx));
        match Pin::new(&mut *guard).poll_flush(cx) {
            Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
            Poll::Ready(Err(error)) => Poll::Ready(Err!(error)),
            Poll::Pending => Poll::Pending,
        }
    }

    fn poll_close(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<()>> {
        let mut guard = futures::ready!(self.inner.lock().poll_unpin(cx));
        match Pin::new(&mut *guard).poll_close(cx) {
            Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
            Poll::Ready(Err(error)) => Poll::Ready(Err!(error)),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<'a> Stream for AsyncWebsocketClient<WebsocketOpen> {
    type Item = Result<Value>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let mut guard = futures::ready!(self.inner.lock().poll_unpin(cx));
        match Pin::new(&mut *guard).poll_next(cx) {
            Poll::Ready(Some(item)) => match item {
                Ok(message) => match message {
                    TungsteniteMessage::Text(response) => match serde_json::from_str(&response) {
                        Ok(response) => Poll::Ready(Some(Ok(response))),
                        Err(error) => Poll::Ready(Some(Err!(error))),
                    },
                    _ => Poll::Ready(Some(Err!(
                        XRPLWebsocketException::<anyhow::Error>::UnexpectedMessageType
                    ))),
                },
                Err(error) => Poll::Ready(Some(Err!(error))),
            },
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<'a> AsyncWebsocketClient<WebsocketClosed> {
    pub async fn open(uri: Url) -> Result<AsyncWebsocketClient<WebsocketOpen>> {
        match tungstenite_connect_async(uri).await {
            Ok((websocket_stream, _)) => Ok(AsyncWebsocketClient {
                inner: Arc::new(Mutex::new(websocket_stream)),
                status: PhantomData::<WebsocketOpen>,
            }),
            Err(error) => {
                Err!(XRPLWebsocketException::UnableToConnect::<anyhow::Error>(
                    error
                ))
            }
        }
    }
}

impl<'a> Client<'a> for AsyncWebsocketClient<WebsocketOpen> {
    async fn request<T>(&self, req: impl Serialize) -> Result<XRPLResponse<'a, T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut this = self.inner.lock().await;
        let request = serde_json::to_string(&req).unwrap();
        this.send(TungsteniteMessage::Text(request)).await.unwrap();
        while let Some(Ok(message)) = this.next().await {
            match message {
                TungsteniteMessage::Text(response) => {
                    let response = serde_json::from_str(&response).unwrap();
                    return Ok(response);
                }
                _ => return Err!(XRPLWebsocketException::<anyhow::Error>::UnexpectedMessageType),
            }
        }

        Err!(XRPLWebsocketException::<anyhow::Error>::NoResponse)
    }

    async fn get_common_fields(&self) -> Result<CommonFields<'a>> {
        let server_state = self
            .request::<results::server_state::ServerState>(requests::ServerState::new(None))
            .await?;
        let state = server_state.result.state;
        let common_fields = CommonFields {
            network_id: state.network_id,
            build_version: Some(state.build_version),
        };

        Ok(common_fields)
    }
}
