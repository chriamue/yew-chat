#[cfg(feature = "yew")]
mod simple_message_handler;

mod message_handler;
#[cfg(feature = "yew")]
mod request_message_handler;

#[cfg(feature = "yew")]
pub use simple_message_handler::SimpleMessageHandler;

#[cfg(feature = "yew")]
pub use request_message_handler::RequestMessageHandler;

pub use message_handler::MessageHandler;
