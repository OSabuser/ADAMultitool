use std::error::Error;
use std::{fmt, io};

/// Ошибка отправки сообщения.
#[derive(Debug)]
pub enum ProtoSendError {
    /// Внутренняя ошибка IO.
    Io(io::Error),
}

impl fmt::Display for ProtoSendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl From<io::Error> for ProtoSendError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl Error for ProtoSendError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        let Self::Io(e) = self;
        Some(e)
    }
}

/// Ошибка приема сообщения.
#[derive(Debug)]
pub enum ProtoRecvError {
    /// Некорректная кодировка принятой строки.
    BadEncoding,

    /// Внутренняя ошибка IO.
    Io(io::Error),
}

impl fmt::Display for ProtoRecvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtoRecvError::BadEncoding => write!(f, "bad encoding"),
            ProtoRecvError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl From<io::Error> for ProtoRecvError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl Error for ProtoRecvError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ProtoRecvError::Io(e) => Some(e),
            ProtoRecvError::BadEncoding => None,
        }
    }
}
