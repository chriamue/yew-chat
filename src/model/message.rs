#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub sender: String,
    pub content: String,
}
