use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SendError {
    UnknownError,
    InternalError(String),
}

impl fmt::Display for SendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SendError::UnknownError => write!(f, "Unknown error occurred"),
            SendError::InternalError(e) => write!(f, "Internal error occurred: {}", e),
        }
    }
}

impl std::error::Error for SendError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReceiveError {
    UnknownError,
    InternalError(String),
}

impl fmt::Display for ReceiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReceiveError::UnknownError => write!(f, "Unknown error occurred"),
            ReceiveError::InternalError(e) => write!(f, "Internal error occurred: {}", e),
        }
    }
}

impl std::error::Error for ReceiveError {}
