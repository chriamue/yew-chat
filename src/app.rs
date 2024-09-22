use async_trait::async_trait;
use std::rc::Rc;
use yew::prelude::*;
use yew_chat::prelude::{Chat, ChatComp, Message, MessageInputComp, MessageSender, SendError};

struct SimpleSender {
    chat: UseStateHandle<Chat>,
}

#[async_trait(?Send)]
impl MessageSender for SimpleSender {
    async fn send_message(&self, message: Message) -> Result<(), SendError> {
        let mut updated_chat = (*self.chat).clone();
        updated_chat.add_message(message);
        self.chat.set(updated_chat);
        Ok(())
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let user = "Me".to_string();
    let chat = use_state(|| Chat::new("channel1".to_string()));

    // Create the sender wrapped in Rc
    let sender = Rc::new(SimpleSender { chat: chat.clone() }) as Rc<dyn MessageSender>;

    html! {
        <div>
            <ChatComp chat={(*chat).clone()} />
            // Clone the Rc itself, not the trait object
            <MessageInputComp current_user={user} sender={sender.clone()} />
        </div>
    }
}
