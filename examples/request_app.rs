use std::sync::Arc;
use yew::prelude::*;
use yew_chat::prelude::{
    Chat, ChatComp, MessageInputComp, MessageReceiver, MessageSender, RequestMessageHandler,
};

#[function_component(App)]
pub fn app() -> Html {
    let user = "Me".to_string();
    let chat = use_state(|| Chat::new("channel1".to_string()));

    let handler = Arc::new(RequestMessageHandler {
        host: "http://localhost:3000".to_string(),
    });

    let sender = handler.clone() as Arc<dyn MessageSender>;
    let receiver = handler.clone() as Arc<dyn MessageReceiver>;

    {
        let receiver = receiver.clone();
        let chat = chat.clone();

        use_effect_with(chat.clone(), move |_| {
            let chat = chat.clone();
            let receiver = receiver.clone();

            let interval = gloo::timers::callback::Interval::new(1000, move || {
                let chat = chat.clone();
                let receiver = receiver.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(messages) = receiver.receive_messages("channel1").await {
                        let mut updated_chat = (*chat).clone();
                        // replace if not found
                        for message in messages {
                            if !updated_chat.messages.iter().any(|m| *m == message) {
                                updated_chat.messages.push(message);
                            }
                        }
                        chat.set(updated_chat);
                    }
                });
            });

            || drop(interval)
        });
    }

    html! {
        <div>
            <ChatComp chat={(*chat).clone()} />
            <MessageInputComp channel={"channel1".to_string()} current_user={user} sender={sender.clone()} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
