use crate::model::{MessageReceiver, MessageSender};

pub trait MessageHandler: MessageSender + MessageReceiver {}
