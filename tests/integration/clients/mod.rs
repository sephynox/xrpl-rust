use anyhow::anyhow;
use anyhow::Result;

#[tokio::test]
#[cfg(feature = "tungstenite")]
async fn test_websocket_tungstenite_echo() -> Result<()> {
    use super::common::connect_to_wss_tungstinite_echo;
    use futures::{SinkExt, TryStreamExt};
    use xrpl::asynch::clients::async_websocket_client::TungsteniteMessage;
    use xrpl::models::requests::AccountInfo;

    let mut websocket = connect_to_wss_tungstinite_echo().await?;
    let account_info = AccountInfo::new(
        "rJumr5e1HwiuV543H7bqixhtFreChWTaHH",
        None,
        None,
        None,
        None,
        None,
        None,
    );

    websocket.send(&account_info).await?;
    while let Ok(Some(TungsteniteMessage::Text(response))) = websocket.try_next().await {
        match serde_json::from_str::<AccountInfo>(response.as_str()) {
            Ok(account_info_echo) => {
                assert_eq!(account_info, account_info_echo);
                return Ok(());
            }
            Err(err) => {
                return Err(anyhow!("Error parsing response: {:?}", err));
            }
        };
    }

    Ok(())
}

#[tokio::test]
#[cfg(feature = "embedded-websocket")]
async fn test_embedded_websocket_echo() -> Result<()> {
    use super::common::{codec::Codec, connect_to_ws_embedded_websocket_tokio_echo};
    use tokio_util::codec::Framed;
    use xrpl::asynch::clients::async_websocket_client::EmbeddedWebsocketReadMessageType;
    use xrpl::models::requests::AccountInfo;

    let tcp_stream = tokio::net::TcpStream::connect("ws.vi-server.org:80")
        .await
        .unwrap();
    let mut framed = Framed::new(tcp_stream, Codec::new());
    let mut buffer = [0u8; 4096];
    let mut websocket =
        connect_to_ws_embedded_websocket_tokio_echo(&mut framed, &mut buffer).await?;
    let account_info = AccountInfo::new(
        "rJumr5e1HwiuV543H7bqixhtFreChWTaHH",
        None,
        None,
        None,
        None,
        None,
        None,
    );
    websocket
        .send(&mut framed, &mut buffer, false, &account_info)
        .await?;

    let mut ping_counter = 0;
    loop {
        match websocket.try_next(&mut framed, &mut buffer).await? {
            Some(message) => match message {
                EmbeddedWebsocketReadMessageType::Ping(_) => {
                    ping_counter += 1;
                    if ping_counter > 1 {
                        panic!("Expected only one ping");
                    }
                }
                EmbeddedWebsocketReadMessageType::Text(text) => {
                    match serde_json::from_str::<AccountInfo>(text) {
                        Ok(account_info_echo) => {
                            assert_eq!(account_info, account_info_echo);
                            return Ok(());
                        }
                        Err(err) => {
                            return Err(anyhow!("Error parsing response: {:?}", err));
                        }
                    }
                }
                EmbeddedWebsocketReadMessageType::Binary(_) => {
                    panic!("Expected text message found binary")
                }
                EmbeddedWebsocketReadMessageType::Pong(_) => {
                    panic!("Expected text message found pong")
                }
                EmbeddedWebsocketReadMessageType::Close(_) => {
                    panic!("Expected text message found close")
                }
            },
            None => return Err(anyhow!("No message received")),
        }
    }
}
