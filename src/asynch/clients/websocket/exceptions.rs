use core::fmt::Debug;
use core::str::Utf8Error;
#[cfg(all(feature = "websocket", not(feature = "std")))]
use embedded_io_async::{Error as EmbeddedIoError, ErrorKind};
#[cfg(all(feature = "websocket", not(feature = "std")))]
use embedded_websocket_embedded_io::framer_async::FramerError;
use thiserror_no_std::Error;

#[derive(Debug, Error)]
pub enum XRPLWebsocketException<E: Debug> {
    // FramerError
    #[error("I/O error: {0:?}")]
    Io(E),
    #[error("Frame too large (size: {0:?})")]
    FrameTooLarge(usize),
    #[error("Failed to interpret u8 to string (error: {0:?})")]
    Utf8(Utf8Error),
    #[error("Invalid HTTP header")]
    HttpHeader,
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
}

#[cfg(all(feature = "websocket", not(feature = "std")))]
impl<E: Debug> From<FramerError<E>> for XRPLWebsocketException<E> {
    fn from(value: FramerError<E>) -> Self {
        match value {
            FramerError::Io(e) => XRPLWebsocketException::Io(e),
            FramerError::FrameTooLarge(e) => XRPLWebsocketException::FrameTooLarge(e),
            FramerError::Utf8(e) => XRPLWebsocketException::Utf8(e),
            FramerError::HttpHeader(_) => XRPLWebsocketException::HttpHeader,
            FramerError::WebSocket(e) => XRPLWebsocketException::WebSocket(e),
            FramerError::Disconnected => XRPLWebsocketException::Disconnected,
            FramerError::RxBufferTooSmall(e) => XRPLWebsocketException::RxBufferTooSmall(e),
        }
    }
}

#[cfg(all(feature = "websocket", not(feature = "std")))]
impl<E: Debug> EmbeddedIoError for XRPLWebsocketException<E> {
    fn kind(&self) -> ErrorKind {
        match self {
            XRPLWebsocketException::EmbeddedIoError(e) => e.kind(),
            _ => ErrorKind::Other,
        }
    }
}

#[cfg(feature = "std")]
impl<E: Debug> alloc::error::Error for XRPLWebsocketException<E> {}
