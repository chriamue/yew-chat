use crate::model::{MessageReceiver, MessageSender};

pub trait MessageStorage: MessageReceiver + MessageSender {}
