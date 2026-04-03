use std::sync::{Arc, Mutex};
use yew::prelude::*;
use yew_chat::prelude::{ChatApp, SimpleMessageHandler};

#[function_component(App)]
pub fn app() -> Html {
    let handler = SimpleMessageHandler {
        message_queue: Arc::new(Mutex::new(Vec::new())),
    };

    html! {
        <ChatApp<SimpleMessageHandler> user="Me" channel="channel1" {handler} />
    }
}
