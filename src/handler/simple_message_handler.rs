use crate::handler::MessageHandler;
use crate::model::{Message, ReceiveError, SendError};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SimpleMessageHandler {
    pub message_queue: Arc<Mutex<Vec<Message>>>,
}

impl PartialEq for SimpleMessageHandler {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.message_queue, &other.message_queue)
    }
}

impl MessageHandler for SimpleMessageHandler {
    async fn send_message(&self, _channel: &str, message: Message) -> Result<(), SendError> {
        self.message_queue.lock().unwrap().push(message);
        Ok(())
    }

    async fn receive_messages(&self, _channel: &str) -> Result<Vec<Message>, ReceiveError> {
        let mut q = self.message_queue.lock().unwrap();
        if q.is_empty() {
            Err(ReceiveError::UnknownError)
        } else {
            Ok(std::mem::take(&mut *q))
        }
    }
}
