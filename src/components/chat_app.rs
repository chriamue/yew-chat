use crate::components::{ChatComp, MessageInputComp};
use crate::handler::MessageHandler;
use crate::model::Chat;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ChatAppProps<H: MessageHandler> {
    pub user: String,
    pub channel: String,
    pub handler: H,
}

#[function_component(ChatApp)]
pub fn chat_app<H: MessageHandler>(props: &ChatAppProps<H>) -> Html {
    let channel = props.channel.clone();
    let handler = props.handler.clone();
    let chat = use_state(|| Chat::new(channel.clone()));

    {
        let chat = chat.clone();
        let channel = channel.clone();
        let handler = handler.clone();

        use_effect_with((), move |_| {
            let interval = gloo::timers::callback::Interval::new(1000, move || {
                let chat = chat.clone();
                let channel = channel.clone();
                let handler = handler.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(messages) = handler.receive_messages(&channel).await {
                        let mut updated = (*chat).clone();
                        for msg in messages {
                            if !updated.messages.contains(&msg) {
                                updated.messages.push(msg);
                            }
                        }
                        updated.messages.sort_by_key(|m| m.timestamp);
                        chat.set(updated);
                    }
                });
            });
            || drop(interval)
        });
    }

    html! {
        <div>
            <ChatComp chat={(*chat).clone()} />
            <MessageInputComp<H> channel={props.channel.clone()} current_user={props.user.clone()} handler={props.handler.clone()} />
        </div>
    }
}
