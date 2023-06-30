use super::common::connect_to_ws_echo;
use xrpl::asynch::clients::{ReadResult, WebsocketClose, WebsocketIo};
use xrpl::models::requests::AccountInfo;

#[tokio::test]
async fn test_websocket_non_tls() {
    let mut buffer = [0u8; 4096];
    let mut websocket = connect_to_ws_echo(&mut buffer).await;
    let account_info = AccountInfo::new(
        "rJumr5e1HwiuV543H7bqixhtFreChWTaHH",
        None,
        None,
        None,
        None,
        None,
        None,
    );
    websocket.write(&account_info).await.unwrap();

    while let Ok(Some(ReadResult::Text(response))) = websocket.read().await {
        let account_info_echo: AccountInfo = serde_json::from_str(response).unwrap();
        assert_eq!(account_info, account_info_echo);

        websocket.close().await.unwrap();
    }
}
