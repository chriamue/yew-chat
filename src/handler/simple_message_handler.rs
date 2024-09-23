use crate::handler::message_handler::MessageHandler;
use crate::model::{Message, MessageReceiver, MessageSender, ReceiveError, SendError};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

pub struct SimpleMessageHandler {
    pub message_queue: Arc<Mutex<Vec<Message>>>,
}

impl MessageHandler for SimpleMessageHandler {}

#[async_trait(?Send)]
impl MessageSender for SimpleMessageHandler {
    async fn send_message(&self, _channel: &str, message: Message) -> Result<(), SendError> {
        let mut queue = self.message_queue.lock().unwrap();
        queue.push(message);
        Ok(())
    }
}

#[async_trait(?Send)]
impl MessageReceiver for SimpleMessageHandler {
    async fn receive_messages(&self, _channel: &str) -> Result<Vec<Message>, ReceiveError> {
        let mut queue = self.message_queue.lock().unwrap();
        if !queue.is_empty() {
            let messages = queue.clone();
            queue.clear();
            Ok(messages)
        } else {
            Err(ReceiveError::UnknownError)
        }
    }
}
