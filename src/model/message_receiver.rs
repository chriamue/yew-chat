use super::Message;
use super::ReceiveError;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait MessageReceiver {
    fn receive_messages(&self, channel: &str) -> Result<Vec<Message>, ReceiveError>;
}
