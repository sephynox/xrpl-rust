use crate::asynch::clients::exceptions::XRPLWebsocketException;
use crate::asynch::clients::websocket_base::WebsocketBase;
use crate::models::Model;
use crate::Err;
use anyhow::Result;
use serde::Serialize;

// exports
#[cfg(feature = "std")]
pub use if_std::AsyncWebsocketClient;

pub use em_as_net::client::websocket::ReadResult;

#[cfg(feature = "std")]
mod if_std {
    use crate::asynch::clients::async_client::AsyncClient;
    use crate::asynch::clients::client::Client;
    use crate::asynch::clients::exceptions::XRPLWebsocketException;
    use crate::asynch::clients::websocket_base::WebsocketBase;
    use crate::models::Model;
    use crate::Err;
    use alloc::borrow::Cow;
    use core::cell::RefCell;
    use core::ops::Deref;

    use anyhow::Result;

    use crate::asynch::clients::Websocket;
    use em_as_net::client::websocket::{
        ReadResult, WebsocketClient, WebsocketClientIo, WebsocketSendMessageType,
    };
    use rand::rngs::ThreadRng;
    use serde::Serialize;
    use tokio::net;

    /// An async client for interacting with the rippled WebSocket API.
    pub struct AsyncWebsocketClient<'a> {
        pub uri: Cow<'a, str>,
        inner: RefCell<Option<WebsocketClient<'a, net::TcpStream, ThreadRng>>>,
    }

    impl<'a> AsyncWebsocketClient<'a> {
        pub fn new(uri: Cow<'a, str>, buffer: &'a mut [u8]) -> Self {
            let ws = WebsocketClient::new(uri.clone(), buffer);
            Self {
                uri,
                inner: RefCell::new(Some(ws)),
            }
        }
    }

    impl<'a> Websocket<'a> for AsyncWebsocketClient<'a> {}

    impl<'a> WebsocketBase<'a> for AsyncWebsocketClient<'a> {
        fn is_open(&self) -> bool {
            if let Some(ws) = self.inner.borrow().deref() {
                ws.is_open()
            } else {
                false
            }
        }

        async fn do_open(&self) -> Result<()> {
            return match self.inner.borrow_mut().as_mut() {
                None => {
                    Err!(XRPLWebsocketException::NotOpen)
                }
                Some(ws) => ws.connect(None).await,
            };
        }

        async fn do_close(&self) -> Result<()> {
            return match self.inner.borrow_mut().as_mut() {
                None => {
                    Err!(XRPLWebsocketException::NotOpen)
                }
                Some(ws) => ws.close().await,
            };
        }

        async fn do_write<T: Model + Serialize>(&self, request: T) -> Result<()> {
            return match self.inner.borrow_mut().as_mut() {
                None => {
                    Err!(XRPLWebsocketException::NotOpen)
                }
                Some(ws) => {
                    let request_string = match serde_json::to_string(&request) {
                        Ok(as_string) => as_string,
                        Err(_) => return Err!(XRPLWebsocketException::RequestSerializationError),
                    };
                    ws.write(
                        Cow::from(request_string),
                        Some(WebsocketSendMessageType::Text),
                    )
                    .await
                }
            };
        }

        // TODO: Fix lifetime issue
        async fn do_read(&'a mut self) -> Result<Option<ReadResult<'a>>> {
            return match self.inner.get_mut() {
                None => {
                    Err!(XRPLWebsocketException::NotOpen)
                }
                Some(ws) => match ws.read().await {
                    None => Ok(None),
                    Some(Ok(read_result)) => Ok(Some(read_result)),
                    Some(Err(read_error)) => Err(read_error),
                },
            };
        }

        async fn do_request_impl<T: Model + Serialize, R>(&mut self, _request: T) -> Result<R> {
            todo!()
        }
    }

    impl<'a> AsyncClient<'a> for AsyncWebsocketClient<'a> {}

    impl<'a> Client<'a> for AsyncWebsocketClient<'a> {
        async fn request_impl<T: Model + Serialize, R>(&mut self, request: T) -> Result<R> {
            if !<AsyncWebsocketClient<'a> as WebsocketBase<'_>>::is_open(self) {
                return Err!(XRPLWebsocketException::NotOpen);
            }

            self.do_request_impl(request).await
        }
    }
}

pub trait Websocket<'a>: WebsocketBase<'a> {
    async fn open(&mut self) -> Result<()> {
        if !self.is_open() {
            self.do_open().await
        } else {
            Ok(())
        }
    }

    async fn close(&self) -> Result<()> {
        if self.is_open() {
            self.do_close().await
        } else {
            Ok(())
        }
    }

    async fn write<T: Model + Serialize>(&mut self, request: T) -> Result<()> {
        if self.is_open() {
            self.do_write(request).await
        } else {
            Err!(XRPLWebsocketException::NotOpen)
        }
    }

    async fn read(&'a mut self) -> Result<Option<ReadResult<'a>>> {
        if self.is_open() {
            self.do_read().await
        } else {
            Err!(XRPLWebsocketException::NotOpen)
        }
    }
}
