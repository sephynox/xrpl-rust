use anyhow::Result;

#[cfg(all(feature = "tungstenite", not(feature = "embedded-ws")))]
pub async fn test_websocket_tungstenite_echo() -> Result<()> {
    use super::common::connect_to_wss_tungstinite_echo;
    use xrpl::{
        asynch::clients::XRPLWebsocketIO, models::requests::Fee, models::results::FeeResult,
    };

    let mut websocket = connect_to_wss_tungstinite_echo().await?;
    let fee = Fee::new(None);

    websocket.xrpl_send(fee).await.unwrap();
    let message = websocket
        .xrpl_receive::<FeeResult<'_>, Fee<'_>>()
        .await
        .unwrap();
    assert!(message.result.is_some());
    Ok(())
}

#[cfg(all(feature = "embedded-ws", not(feature = "tungstenite")))]
pub async fn test_embedded_websocket_echo() -> Result<()> {
    use super::common::connect_to_ws_embedded_websocket_tokio_echo;
    use tokio_util::codec::Framed;
    use xrpl::asynch::clients::codec::Codec;
    use xrpl::asynch::clients::XRPLWebsocketIO;
    use xrpl::models::requests::Fee;
    use xrpl::models::results::FeeResult;

    let tcp_stream = tokio::net::TcpStream::connect("ws.vi-server.org:80")
        .await
        .unwrap();
    let mut framed = Framed::new(tcp_stream, Codec);
    let mut websocket = connect_to_ws_embedded_websocket_tokio_echo(framed).await?;
    let fee = Fee::new(None);
    websocket.xrpl_send(fee).await?;
    let _ = websocket
        .xrpl_receive::<FeeResult<'_>, Fee<'_>>()
        .await
        .unwrap();
    Ok(())
}
