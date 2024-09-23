use super::Message;
use super::ReceiveError;
use async_trait::async_trait;

#[cfg(not(feature = "yew"))]
#[async_trait]
pub trait MessageReceiver: Send + Sync {
    async fn receive_messages(&self, channel: &str) -> Result<Vec<Message>, ReceiveError>;
}

#[cfg(feature = "yew")]
#[async_trait(?Send)]
pub trait MessageReceiver {
    async fn receive_messages(&self, channel: &str) -> Result<Vec<Message>, ReceiveError>;
}
