use std::fmt;

#[derive(Debug)]
pub enum SendError {
    UnknownError,
}

impl fmt::Display for SendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SendError::UnknownError => write!(f, "Unknown error occurred"),
        }
    }
}

impl std::error::Error for SendError {}

#[derive(Debug)]
pub enum ReceiveError {
    UnknownError,
}

impl fmt::Display for ReceiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReceiveError::UnknownError => write!(f, "Unknown error occurred"),
        }
    }
}

impl std::error::Error for ReceiveError {}
