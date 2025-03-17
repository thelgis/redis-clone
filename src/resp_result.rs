use std::fmt;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum RESPError {
    FromUtf8,
    OutOfBounds(usize),
}

impl From<FromUtf8Error> for RESPError {
    fn from(_: FromUtf8Error) -> Self {
        Self::FromUtf8
    }
}

impl fmt::Display for RESPError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RESPError::FromUtf8 => write!(f, "Cannot convert from UTF-8"),
            RESPError::OutOfBounds(index) => write!(f, "Out of bounds at index {}", index),
        }
    }
}

pub type RESPResult<T> = Result<T, RESPError>;
