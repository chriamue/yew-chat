use super::Message;
use super::SendError;
use async_trait::async_trait;

#[async_trait]
pub trait MessageSender: Send + Sync {
    async fn send_message(&self, channel: &str, message: Message) -> Result<(), SendError>;
}
