use super::Message;
use super::ReceiveError;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait MessageReceiver {
    fn receive_message(&self) -> Result<Message, ReceiveError>;
}
