use std::io;
use thiserror::Error;

/// Ошибки в формате сообщения
#[derive(Error, PartialEq, Debug)]
pub enum FrameDecodeError {
    #[error("Bad payload size")]
    BadDataSize,
    #[error("Bad payload decoding")]
    BadEncoding,
    #[error("Bad frame CRC")]
    BadCRC,
    #[error("Bad frame prefix")]
    BadPrefix,
    #[error("Bad frame postfix")]
    BadPostfix,
}

/// Ошибка приема сообщения.
#[derive(Error, Debug)]
pub enum ProtoRecvError {
    #[error("Failed to decode message")]
    Decode(#[from] FrameDecodeError),
    /// Внутренняя ошибка IO.
    #[error("Internal IO error")]
    Io(#[from] io::Error),
}

/// Ошибка приема сообщения.
#[derive(Error, Debug)]
pub enum ProtoSendError {
    /// Внутренняя ошибка IO.
    #[error("Internal IO error")]
    Io(#[from] io::Error),
}

/// Ошибка обмена данными с устройством
#[derive(Error, Debug)]
pub enum DeviceConnectError {
    #[error("Bad handshake!")]
    BadHandshake,
}
