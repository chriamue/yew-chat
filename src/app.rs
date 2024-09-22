use yew::prelude::*;
use yew_chat::prelude::{Chat, Input, Message, MessageComp};

#[function_component(App)]
pub fn app() -> Html {
    let chat = use_state(|| Chat::new("channel1".to_string()));

    let on_submit = {
        let chat = chat.clone();
        Callback::from(move |content: String| {
            let message = Message {
                sender: "Me".to_string(),
                content,
                timestamp: chrono::Local::now().to_rfc2822(),
            };
            let mut updated_chat: Chat = (*chat).clone();
            updated_chat.add_message(message);
            chat.set(updated_chat);
        })
    };

    html! {
        <div>
            <div class="chat">
                {for chat.get_messages().iter().map(|message| {
                    html! {
                        <MessageComp message={message.clone()} />
                    }
                })}
            </div>
            <Input {on_submit} />
        </div>
    }
}
