use crate::model::{Message, MessageReceiver, MessageSender, ReceiveError, SendError};
use crate::server::MessageStorage;
use async_trait::async_trait;
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
}

#[async_trait]
impl MessageSender for MemoryMessageStorage {
    async fn send_message(&self, channel: &str, message: Message) -> Result<(), SendError> {
        let mut storage = self.storage.lock().await;
        storage
            .entry(channel.to_string())
            .or_default()
            .push(message);
        Ok(())
    }
}

#[async_trait]
impl MessageReceiver for MemoryMessageStorage {
    async fn receive_messages(&self, channel: &str) -> Result<Vec<Message>, ReceiveError> {
        let storage = self.storage.lock().await;
        let messages = storage.get(channel).cloned().unwrap_or_default();
        Ok(messages)
    }
}

impl MessageStorage for MemoryMessageStorage {}
