use std::sync::{Arc, Mutex};
use yew::prelude::*;
use yew_chat::prelude::{ChatApp, MessageHandler, SimpleMessageHandler};

#[function_component(App)]
pub fn app() -> Html {
    let user = "Me".to_string();
    let channel = "channel1".to_string();

    let message_queue = Arc::new(Mutex::new(Vec::new()));

    let handler = Arc::new(SimpleMessageHandler {
        message_queue: message_queue.clone(),
    }) as Arc<dyn MessageHandler>;

    html! {
        <div>
            <ChatApp user={user.clone()} channel={channel.clone()} handler={handler.clone()}/>
        </div>
    }
}
