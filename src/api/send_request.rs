use crate::model::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(utoipa::ToSchema))]
pub struct SendRequest {
    pub message: Message,
}
