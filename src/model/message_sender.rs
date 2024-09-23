use super::Message;
use super::SendError;
use async_trait::async_trait;

#[cfg(not(feature = "yew"))]
#[async_trait]
pub trait MessageSender: Send {
    async fn send_message(&self, channel: &str, message: Message) -> Result<(), SendError>;
}

#[cfg(feature = "yew")]
#[async_trait(?Send)]
pub trait MessageSender {
    async fn send_message(&self, channel: &str, message: Message) -> Result<(), SendError>;
}
