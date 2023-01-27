// use futures::StreamExt;
// use tokio_tungstenite::tungstenite::Message;
// use xrpl::models::{
//     requests::{AccountInfo, Subscribe},
//     StreamParameter,
// };
// use xrpl::tokio::AsyncWebsocketClient;

/// Setup common testing prerequisites here such as connecting a client
/// to a server or creating required files/directories if needed.
pub fn setup() {}

// #[tokio::test]
// async fn test_client_send() {
//     let client = AsyncWebsocketClient {
//         url: "wss://xrplcluster.com/",
//     };
//     let (mut ws_stream, (sender, mut receiver)) = client.open().await.unwrap();

//     tokio::spawn(async move {
//         while let Some(msg) = receiver.next().await {
//             assert!(msg.is_ok());
//             receiver.close();
//             break;
//         }
//     });

//     let request = Subscribe {
//         streams: Some(vec![StreamParameter::Ledger]),
//         ..Default::default()
//     };
//     let message = Message::Text(serde_json::to_string(&request).unwrap());

//     match client.send(&mut ws_stream, sender, message).await {
//         Ok(_) => (),
//         Err(_error) => (),
//     }
// }

// #[tokio::test]
// async fn test_client_request() {
//     let client = AsyncWebsocketClient {
//         url: "wss://xrplcluster.com/",
//     };

//     let request = AccountInfo {
//         account: "r3rhWeE31Jt5sWmi4QiGLMZnY3ENgqw96W",
//         ..Default::default()
//     };
//     let message = Message::Text(serde_json::to_string(&request).unwrap());

//     assert!(client.request(message).await.is_ok());
// }
