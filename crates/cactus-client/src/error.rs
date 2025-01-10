use cactus_protocol::Message;
use cactus_traits::cactus_error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error<R> {
    #[error("{0:?}")]
    Cactus(#[from] cactus_error::Error),

    #[error("{0}")]
    WebSocket(#[from] tungstenite::Error),

    #[error("{0:?}")]
    InvalidResponse(Message),

    #[error("missing response")]
    MissingResponse,

    #[error("rejection")]
    Rejection(R),
}
