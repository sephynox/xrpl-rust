use super::{
    exceptions::XRPLWebsocketException, Client, CommonFields, WebsocketClient, WebsocketClosed,
    WebsocketOpen, XRPLResponse,
};
use crate::Err;

use alloc::sync::Arc;
use anyhow::Result;
use core::marker::PhantomData;
use core::{pin::Pin, task::Poll};
use futures::{Sink, Stream};
use futures_util::{SinkExt, TryStreamExt};
use serde::Serialize;
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
    type Item = Result<XRPLResponse>;

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
    async fn request(&mut self, req: impl Serialize) -> Result<XRPLResponse> {
        self.send(req).await?;
        while let Ok(Some(response)) = self.try_next().await {
            return Ok(response);
        }

        Err!(XRPLWebsocketException::<anyhow::Error>::NoResponse)
    }

    async fn get_common_fields(self) -> Result<CommonFields<'a>> {
        todo!()
    }

    async fn set_common_fields(&mut self, common_fields_response: &XRPLResponse) -> Result<()> {
        let result = common_fields_response.result.clone();

        let network_id = result
            .as_ref()
            .and_then(|result| {
                result
                    .get("info")
                    .and_then(|info| info.get("network_id"))
                    .and_then(|network_id| network_id.as_str())
                    .map(|network_id| network_id.into())
            })
            .clone();

        let build_version = result
            .as_ref()
            .and_then(|result| {
                result
                    .get("info")
                    .and_then(|info| info.get("build_version"))
                    .and_then(|build_version| build_version.as_str())
                    .map(|build_version| build_version.into())
            })
            .clone();

        self.common_fields = Some(CommonFields {
            network_id,
            build_version,
        });

        Ok(())
    }
}
