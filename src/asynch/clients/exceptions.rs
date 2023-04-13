use thiserror_no_std::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XRPLWebsocketException {
    #[error("Websocket is not open")]
    NotOpen,
    #[error("Failed to serialize XRPL request to string")]
    RequestSerializationError,
}
