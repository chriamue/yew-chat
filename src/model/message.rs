use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(ToSchema))]
pub struct Message {
    pub id: Option<String>,
    #[cfg_attr(
        feature = "server",
        schema(
            example = "2024-09-23T06:42:28Z",
            value_type = String,
            format = "date-time"
        )
    )]
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub sender: String,
    pub content: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_serde() {
        let message = Message {
            id: Some("123".to_string()),
            timestamp: chrono::Utc::now(),
            sender: "Alice".to_string(),
            content: "Hello, Bob!".to_string(),
        };

        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: Message = serde_json::from_str(&serialized).unwrap();

        assert_eq!(message.id, deserialized.id);
        assert_eq!(message.sender, deserialized.sender);
        assert_eq!(message.content, deserialized.content);

        // Compare timestamps allowing for small differences in precision
        let time_diff = (message.timestamp - deserialized.timestamp).num_milliseconds();
        assert!(
            time_diff.abs() < 1,
            "Timestamps differ by more than 1 millisecond"
        );
    }
}
