use super::{
    exceptions::XRPLWebsocketException, Client, CommonFields, WebsocketClient, WebsocketClosed,
    WebsocketOpen,
};
use crate::models::requests;
use crate::models::results::{self, XRPLResponse};
use crate::Err;

use crate::models::results::XRPLResponseFromStream;
use anyhow::Result;
use core::marker::PhantomData;
use core::{pin::Pin, task::Poll};
use futures::{Sink, Stream};
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

pub struct AsyncWebsocketClient<'a, Status = WebsocketClosed> {
    common_fields: Option<CommonFields<'a>>,
    inner: TungsteniteWebsocketStream<TungsteniteMaybeTlsStream<TcpStream>>,
    status: PhantomData<Status>,
}

impl<'a, Status> WebsocketClient<Status> for AsyncWebsocketClient<'a, Status> {}

impl<'a, I> Sink<I> for AsyncWebsocketClient<'a, WebsocketOpen>
where
    I: serde::Serialize,
{
    type Error = anyhow::Error;

    fn poll_ready(
        mut self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<()>> {
        match Pin::new(&mut self.inner).poll_ready(cx) {
            Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
            Poll::Ready(Err(error)) => Poll::Ready(Err!(error)),
            Poll::Pending => Poll::Pending,
        }
    }

    fn start_send(mut self: core::pin::Pin<&mut Self>, item: I) -> Result<()> {
        match serde_json::to_string(&item) {
            Ok(json) => {
                match Pin::new(&mut self.inner).start_send(TungsteniteMessage::Text(json)) {
                    Ok(()) => Ok(()),
                    Err(error) => Err!(error),
                }
            }
            Err(error) => Err!(error),
        }
    }

    fn poll_flush(
        mut self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<()>> {
        match Pin::new(&mut self.inner).poll_flush(cx) {
            Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
            Poll::Ready(Err(error)) => Poll::Ready(Err!(error)),
            Poll::Pending => Poll::Pending,
        }
    }

    fn poll_close(
        mut self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<()>> {
        match Pin::new(&mut self.inner).poll_close(cx) {
            Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
            Poll::Ready(Err(error)) => Poll::Ready(Err!(error)),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<'a> Stream for AsyncWebsocketClient<'a, WebsocketOpen> {
    type Item = Result<Value>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.inner).poll_next(cx) {
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

impl<'a> AsyncWebsocketClient<'a, WebsocketClosed> {
    pub async fn open(uri: Url) -> Result<AsyncWebsocketClient<'a, WebsocketOpen>> {
        match tungstenite_connect_async(uri).await {
            Ok((websocket_stream, _)) => Ok(AsyncWebsocketClient {
                common_fields: None,
                inner: websocket_stream,
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

impl<'a> Client<'a> for AsyncWebsocketClient<'a, WebsocketOpen> {
    async fn request<T>(&mut self, req: impl Serialize) -> Result<XRPLResponse<'_, T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.send(req).await?;
        while let Ok(Some(response)) = self.try_next_xrpl_response().await {
            return Ok(response);
        }

        Err!(XRPLWebsocketException::<anyhow::Error>::NoResponse)
    }

    fn get_common_fields(&self) -> Option<CommonFields<'a>> {
        self.common_fields.clone()
    }

    async fn set_common_fields(&mut self) -> Result<()> {
        let server_state = self
            .request::<results::server_state::ServerState>(requests::ServerState::new(None))
            .await?;
        let state = server_state.result.state.clone();
        self.common_fields = Some(CommonFields {
            network_id: state.network_id,
            build_version: Some(state.build_version),
        });

        Ok(())
    }
}
