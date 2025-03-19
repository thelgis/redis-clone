use std::fmt;
use std::string::FromUtf8Error;

#[derive(Debug, PartialEq)]
pub enum RESPError {
    FromUtf8,
    OutOfBounds(usize),
    WrongType,
    Unknown,
}

impl From<FromUtf8Error> for RESPError {
    fn from(_: FromUtf8Error) -> Self {
        Self::FromUtf8
    }
}

impl fmt::Display for RESPError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RESPError::OutOfBounds(index) => write!(f, "Out of bounds at index {}", index),
            RESPError::FromUtf8 => write!(f, "Cannot convert from UTF-8"),
            RESPError::WrongType => write!(f, "Wrong prefix for RESP type"),
            RESPError::Unknown => write!(f, "Unknown format for RESP string"),
        }
    }
}

pub type RESPResult<T> = Result<T, RESPError>;
