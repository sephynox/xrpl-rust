#[cfg(feature = "std")]
mod std_test {
    use xrpl::asynch::clients::{AsyncWebsocketClient, ReadResult, Websocket};
    use xrpl::models::requests::AccountInfo;

    #[tokio::test]
    async fn test_async_ws() {
        let mut buffer = [0u8; 4096];
        let uri = "ws://limpidcrypto.de:6004";
        let mut ws = AsyncWebsocketClient::new(uri.into(), &mut buffer);
        // connect
        ws.open().await.unwrap();
        // send request
        let account_info = AccountInfo::new(
            "rJumr5e1HwiuV543H7bqixhtFreChWTaHH",
            None,
            None,
            None,
            None,
            None,
            None,
        );
        ws.write(account_info).await.unwrap();
        // read messages
        while let Ok(Some(ReadResult::Text(response))) = ws.read().await {
            println!("{:?}", response);

            break;
        }
    }
}
