use crate::model::{Message, ReceiveError, SendError};

#[allow(async_fn_in_trait)]
pub trait MessageHandler: Clone + PartialEq + 'static {
    async fn send_message(&self, channel: &str, message: Message) -> Result<(), SendError>;
    async fn receive_messages(&self, channel: &str) -> Result<Vec<Message>, ReceiveError>;
}
