use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
