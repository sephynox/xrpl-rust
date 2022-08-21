use futures_channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, tungstenite::Error, MaybeTlsStream,
    WebSocketStream,
};

pub struct AsyncWebsocketClient<'a> {
    pub url: &'a str,
}

impl AsyncWebsocketClient<'static> {
    pub async fn open(
        &self,
    ) -> Result<
        (
            WebSocketStream<MaybeTlsStream<TcpStream>>,
            (
                UnboundedSender<Result<Message, Error>>,
                UnboundedReceiver<Result<Message, Error>>,
            ),
        ),
        Error,
    > {
        let ws_stream = connect_async(self.url).await;
        match ws_stream {
            Ok((result, _)) => Ok((result, unbounded())),
            Err(error) => Err(error),
        }
    }

    async fn _do_send(
        &self,
        stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
        request: Message,
    ) -> Result<(), Error> {
        stream.send(request).await
    }

    pub async fn send(
        &self,
        stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
        channel_sender: UnboundedSender<Result<Message, Error>>,
        request: Message,
    ) -> Result<(), Error> {
        // TODO: Add a method to turn a request into a `Message`.
        let result = self._do_send(stream, request).await;
        match result {
            Ok(_) => {
                // TODO: Improve error handling.
                while let Some(message) = stream.next().await {
                    match channel_sender.unbounded_send(message) {
                        Ok(_) => (),
                        Err(_error) => {
                            // receiver side channel is closed.
                            channel_sender.close_channel(); // close whole channel
                            stream
                                .close(None)
                                .await
                                .expect("Could not close or already closed websocket stream");
                            break;
                        }
                    }
                }
                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    pub async fn request(&self, request: Message) -> Result<Message, Error> {
        let open = self.open().await;
        match open {
            Ok((mut stream, (_sender, mut _receiver))) => {
                _sender.close_channel();
                _receiver.close();
                let result = self._do_send(&mut stream, request).await;
                match result {
                    Ok(_) => match stream.next().await {
                        Some(result) => match result {
                            Ok(message) => Ok(message),
                            Err(error) => Err(error),
                        },
                        None => Err(Error::AlreadyClosed),
                    },
                    Err(error) => match stream.close(None).await {
                        Ok(_) => Err(error),
                        Err(close_error) => Err(close_error),
                    },
                }
            }
            Err(error) => Err(error),
        }
    }
}

#[tokio::test]
async fn test_client_open() {
    let client = AsyncWebsocketClient {
        url: "wss://xrplcluster.com/",
    };
    let result = client.open().await;

    assert!(result.is_ok());
}
