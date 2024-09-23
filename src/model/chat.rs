use super::Message;

#[derive(Debug, Clone, PartialEq)]
pub struct Chat {
    pub channel: String,
    pub messages: Vec<Message>,
}

impl Chat {
    pub fn new(channel: String) -> Self {
        Chat {
            channel,
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn get_messages(&self) -> &Vec<Message> {
        &self.messages
    }
}
