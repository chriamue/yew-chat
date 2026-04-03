use super::Message;
use super::ReceiveError;

#[cfg(not(feature = "yew"))]
#[allow(async_fn_in_trait)]
pub trait MessageReceiver: Send {
    async fn receive_messages(&self, channel: &str) -> Result<Vec<Message>, ReceiveError>;
}

#[cfg(feature = "yew")]
#[allow(async_fn_in_trait)]
pub trait MessageReceiver {
    async fn receive_messages(&self, channel: &str) -> Result<Vec<Message>, ReceiveError>;
}
