use crate::model::{Message, ReceiveError, SendError};
use std::collections::HashMap;
use tokio::sync::Mutex;

pub struct MemoryMessageStorage {
    storage: Mutex<HashMap<String, Vec<Message>>>,
}

impl MemoryMessageStorage {
    pub fn new() -> Self {
        MemoryMessageStorage {
            storage: Mutex::new(HashMap::new()),
        }
    }

    pub async fn send_message(&self, channel: &str, message: Message) -> Result<(), SendError> {
        self.storage
            .lock()
            .await
            .entry(channel.to_string())
            .or_default()
            .push(message);
        Ok(())
    }

    pub async fn receive_messages(&self, channel: &str) -> Result<Vec<Message>, ReceiveError> {
        Ok(self
            .storage
            .lock()
            .await
            .get(channel)
            .cloned()
            .unwrap_or_default())
    }
}
