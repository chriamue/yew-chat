use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub sender: String,
    pub content: String,
}
