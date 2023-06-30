use rand::rngs::ThreadRng;
use xrpl::asynch::clients::{
    AsyncWebsocketClient, Open, TcpAdapterTokio, TcpSocket, WebsocketBase, WebsocketOpen,
};

mod constants;
pub use constants::*;

pub async fn connect_to_ws_echo<'a>(
    buffer: &'a mut [u8],
) -> AsyncWebsocketClient<'a, TcpSocket<TcpAdapterTokio>, ThreadRng, Open> {
    let websocket = AsyncWebsocketClient::new(ECHO_WS_SERVER.into(), buffer);
    let tcp_adapter = TcpAdapterTokio::new();

    let websocket = websocket.open(tcp_adapter).await.unwrap();
    assert!(websocket.is_open());

    websocket
}
