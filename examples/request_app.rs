use yew::prelude::*;
use yew_chat::prelude::{ChatApp, RequestMessageHandler};

#[function_component(App)]
pub fn app() -> Html {
    let handler = RequestMessageHandler {
        host: "http://localhost:3000".to_string(),
    };

    html! {
        <ChatApp<RequestMessageHandler> user="Me" channel="channel1" {handler} />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
