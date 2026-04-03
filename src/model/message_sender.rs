use super::Message;
use super::SendError;

#[cfg(not(feature = "yew"))]
#[allow(async_fn_in_trait)]
pub trait MessageSender: Send {
    async fn send_message(&self, channel: &str, message: Message) -> Result<(), SendError>;
}

#[cfg(feature = "yew")]
#[allow(async_fn_in_trait)]
pub trait MessageSender {
    async fn send_message(&self, channel: &str, message: Message) -> Result<(), SendError>;
}
