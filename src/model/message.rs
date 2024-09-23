use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(ToSchema))]
pub struct Message {
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
