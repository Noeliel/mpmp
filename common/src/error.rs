use std::fmt::Debug;

#[derive(Debug)]
pub enum Error {
    OtherError(&'static str),
    MiscError(String),
    LockingFailedError,
    SocketClosedError,
    IOError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IOError(value)
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        Error::LockingFailedError
    }
}
