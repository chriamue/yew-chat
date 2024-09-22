#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub timestamp: String,
    pub sender: String,
    pub content: String,
}
