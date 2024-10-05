use alloc::{format, string::String};
use core::{fmt::Debug, str::Utf8Error};
#[cfg(all(feature = "websocket", not(feature = "std")))]
use embedded_io_async::{Error as EmbeddedIoError, ErrorKind};
#[cfg(all(feature = "websocket", not(feature = "std")))]
use embedded_websocket_embedded_io::framer_async::FramerError;
use futures::channel::oneshot::Canceled;
use thiserror_no_std::Error;

pub type XRPLWebSocketResult<T> = Result<T, XRPLWebSocketException>;

#[derive(Debug, Error)]
pub enum XRPLWebSocketException {
    // FramerError
    #[error("I/O error: {0:?}")]
    Io(String),
    #[error("Frame too large (size: {0:?})")]
    FrameTooLarge(usize),
    #[error("{0:?}")]
    Utf8(#[from] Utf8Error),
    #[error("Invalid HTTP header")]
    HttpHeader,
    #[cfg(all(feature = "websocket", feature = "std"))]
    #[error("Websocket error: {0:?}")]
    TungsteniteError(#[from] tokio_tungstenite::tungstenite::Error),
    #[cfg(all(feature = "websocket", not(feature = "std")))]
    #[error("Websocket error: {0:?}")]
    WebSocket(embedded_websocket_embedded_io::Error),
    #[error("Disconnected")]
    Disconnected,
    #[error("Read buffer is too small (size: {0:?})")]
    RxBufferTooSmall(usize),
    #[error("Unexpected message type")]
    UnexpectedMessageType,
    #[cfg(all(feature = "websocket", not(feature = "std")))]
    #[error("Embedded I/O error: {0:?}")]
    EmbeddedIoError(ErrorKind),
    #[error("Missing request channel sender.")]
    MissingRequestSender,
    #[error("Missing request channel receiver.")]
    MissingRequestReceiver,
    #[error("Invalid message.")]
    InvalidMessage,
    #[error("0:?")]
    SerdeError(#[from] serde_json::Error),
    #[error("0:?")]
    CancelledRequest(Canceled),
}

#[cfg(all(feature = "websocket", not(feature = "std")))]
impl<E: Debug> From<FramerError<E>> for XRPLWebSocketException {
    fn from(value: FramerError<E>) -> Self {
        match value {
            FramerError::Io(e) => XRPLWebSocketException::Io(format!("{:?}", e)),
            FramerError::FrameTooLarge(e) => XRPLWebSocketException::FrameTooLarge(e),
            FramerError::Utf8(e) => XRPLWebSocketException::Utf8(e),
            FramerError::HttpHeader(_) => XRPLWebSocketException::HttpHeader,
            FramerError::WebSocket(e) => XRPLWebSocketException::WebSocket(e),
            FramerError::Disconnected => XRPLWebSocketException::Disconnected,
            FramerError::RxBufferTooSmall(e) => XRPLWebSocketException::RxBufferTooSmall(e),
        }
    }
}

#[cfg(all(feature = "websocket", not(feature = "std")))]
impl EmbeddedIoError for XRPLWebSocketException {
    fn kind(&self) -> ErrorKind {
        match self {
            XRPLWebSocketException::EmbeddedIoError(e) => e.kind(),
            _ => ErrorKind::Other,
        }
    }
}

#[cfg(feature = "std")]
impl alloc::error::Error for XRPLWebSocketException {}
