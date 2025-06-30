use super::exceptions::XRPLWebSocketException;
use super::{WebSocketClosed, WebSocketOpen};
use crate::asynch::clients::client::XRPLClient;
use crate::asynch::clients::exceptions::{XRPLClientException, XRPLClientResult};
use crate::asynch::clients::websocket::websocket_base::{MessageHandler, WebsocketBase};
use crate::asynch::clients::SingleExecutorMutex;
use crate::models::requests::{Request, XRPLRequest};

use alloc::string::{String, ToString};
use alloc::sync::Arc;
use core::marker::PhantomData;
use core::{pin::Pin, task::Poll};
use embassy_futures::block_on;
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::mutex::Mutex;
use futures::{Sink, SinkExt, Stream, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite, MaybeTlsStream, WebSocketStream};
use url::Url;

use tokio_tungstenite::connect_async as tokio_tungstenite_connect_async;

type TokioTungsteniteMaybeTlsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct AsyncWebSocketClient<M = SingleExecutorMutex, Status = WebSocketClosed>
where
    M: RawMutex,
{
    websocket: Arc<Mutex<M, TokioTungsteniteMaybeTlsStream>>,
    websocket_base: Arc<Mutex<M, WebsocketBase<M>>>,
    uri: Url,
    status: PhantomData<Status>,
}

impl<M> Sink<String> for AsyncWebSocketClient<M, WebSocketOpen>
where
    M: RawMutex,
{
    type Error = XRPLClientException;

    fn poll_ready(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<XRPLClientResult<()>> {
        let mut guard = block_on(self.websocket.lock());
        match Pin::new(&mut *guard).poll_ready(cx) {
            Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
            Poll::Ready(Err(error)) => Poll::Ready(Err(error.into())),
            Poll::Pending => Poll::Pending,
        }
    }

    fn start_send(self: core::pin::Pin<&mut Self>, item: String) -> XRPLClientResult<()> {
        let mut guard = block_on(self.websocket.lock());
        match Pin::new(&mut *guard).start_send(tungstenite::Message::Text(item)) {
            Ok(()) => Ok(()),
            Err(error) => Err(error.into()),
        }
    }

    fn poll_flush(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<XRPLClientResult<()>> {
        let mut guard = block_on(self.websocket.lock());
        match Pin::new(&mut *guard).poll_flush(cx) {
            Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
            Poll::Ready(Err(error)) => Poll::Ready(Err(error.into())),
            Poll::Pending => Poll::Pending,
        }
    }

    fn poll_close(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<XRPLClientResult<()>> {
        let mut guard = block_on(self.websocket.lock());
        match Pin::new(&mut *guard).poll_close(cx) {
            Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
            Poll::Ready(Err(error)) => Poll::Ready(Err(error.into())),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<M> Stream for AsyncWebSocketClient<M, WebSocketOpen>
where
    M: RawMutex,
{
    type Item = XRPLClientResult<String>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let mut guard = block_on(self.websocket.lock());
        match Pin::new(&mut *guard).poll_next(cx) {
            Poll::Ready(Some(item)) => match item {
                Ok(message) => match message {
                    tungstenite::Message::Text(response) => Poll::Ready(Some(Ok(response))),
                    tungstenite::Message::Binary(response) => {
                        let response_string = match String::from_utf8(response) {
                            Ok(string) => string,
                            Err(error) => {
                                return Poll::Ready(Some(Err(XRPLWebSocketException::Utf8(
                                    error.utf8_error(),
                                )
                                .into())));
                            }
                        };
                        Poll::Ready(Some(Ok(response_string)))
                    }
                    tungstenite::Message::Close(_) => {
                        Poll::Ready(Some(Err(XRPLWebSocketException::Disconnected.into())))
                    }
                    _ => Poll::Ready(Some(Err(
                        XRPLWebSocketException::UnexpectedMessageType.into()
                    ))),
                },
                Err(error) => Poll::Ready(Some(Err(error.into()))),
            },
            Poll::Ready(None) => {
                Poll::Ready(Some(Err(XRPLWebSocketException::Disconnected.into())))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<M> AsyncWebSocketClient<M, WebSocketClosed>
where
    M: RawMutex,
{
    pub async fn open(uri: Url) -> XRPLClientResult<AsyncWebSocketClient<M, WebSocketOpen>> {
        let stream = match tokio_tungstenite_connect_async(uri.to_string()).await {
            Ok((stream, _)) => stream,
            Err(error) => return Err(error.into()),
        };
        Ok(AsyncWebSocketClient {
            websocket: Arc::new(Mutex::new(stream)),
            websocket_base: Arc::new(Mutex::new(WebsocketBase::new())),
            uri,
            status: PhantomData::<WebSocketOpen>,
        })
    }
}

impl<M> AsyncWebSocketClient<M, WebSocketOpen>
where
    M: RawMutex,
{
    pub async fn close(&self) -> XRPLClientResult<()> {
        let mut websocket = self.websocket.lock().await;
        let mut websocket_base = self.websocket_base.lock().await;
        websocket_base.close();
        match websocket.close(None).await {
            Ok(()) => Ok(()),
            Err(error) => Err(error.into()),
        }
    }
}

impl<M, Status> AsyncWebSocketClient<M, Status>
where
    M: RawMutex,
{
    pub fn is_open(&self) -> bool {
        core::any::type_name::<Status>() == core::any::type_name::<WebSocketOpen>()
    }
}

impl<M> MessageHandler for AsyncWebSocketClient<M, WebSocketOpen>
where
    M: RawMutex,
{
    async fn setup_request_future(&mut self, id: String) {
        let mut websocket_base = self.websocket_base.lock().await;
        websocket_base.setup_request_future(id).await;
    }

    async fn handle_message(&mut self, message: String) -> XRPLClientResult<()> {
        let mut websocket_base = self.websocket_base.lock().await;
        websocket_base.handle_message(message).await
    }

    async fn pop_message(&mut self) -> String {
        let mut websocket_base = self.websocket_base.lock().await;
        websocket_base.pop_message().await
    }

    async fn try_recv_request(&mut self, id: String) -> XRPLClientResult<Option<String>> {
        let mut websocket_base = self.websocket_base.lock().await;
        websocket_base.try_recv_request(id).await
    }
}

impl<M> XRPLClient for AsyncWebSocketClient<M, WebSocketOpen>
where
    M: RawMutex,
{
    fn get_host(&self) -> Url {
        self.uri.clone()
    }

    async fn request_impl<'a: 'b, 'b>(
        &self,
        mut request: XRPLRequest<'a>,
    ) -> XRPLClientResult<String> {
        // setup request future
        self.set_request_id(&mut request);
        let request_id = request.get_common_fields().id.as_ref().unwrap();
        let mut websocket_base = self.websocket_base.lock().await;
        websocket_base
            .setup_request_future(request_id.to_string())
            .await;
        // send request
        let mut websocket = self.websocket.lock().await;
        let request_string = match serde_json::to_string(&request) {
            Ok(request_string) => request_string,
            Err(error) => return Err(error.into()),
        };
        if let Err(error) = websocket
            .send(tungstenite::Message::Text(request_string))
            .await
        {
            return Err(error.into());
        }
        // wait for response
        loop {
            let message = websocket.next().await;
            match message {
                Some(Ok(tungstenite::Message::Text(message))) => {
                    websocket_base.handle_message(message).await?;
                    let message_opt = websocket_base
                        .try_recv_request(request_id.to_string())
                        .await?;
                    if let Some(message) = message_opt {
                        return Ok(message);
                    }
                }
                Some(Ok(tungstenite::Message::Binary(response))) => {
                    let message = match String::from_utf8(response) {
                        Ok(string) => string,
                        Err(error) => {
                            return Err(XRPLWebSocketException::Utf8(error.utf8_error()).into());
                        }
                    };
                    match serde_json::from_str(&message) {
                        Ok(response) => return Ok(response),
                        Err(error) => return Err(error.into()),
                    }
                }
                Some(Ok(tungstenite::Message::Close(_))) => {
                    return Err(XRPLWebSocketException::Disconnected.into());
                }
                Some(Ok(_)) => {
                    return Err(XRPLWebSocketException::UnexpectedMessageType.into());
                }
                Some(Err(error)) => return Err(error.into()),
                None => continue,
            }
        }
    }
}
