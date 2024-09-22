use yew::prelude::*;
use yew_chat::prelude::{Input, Message, MessageComp};

#[function_component(App)]
fn app() -> Html {
    let messages = use_state(|| Vec::<Message>::new());

    let on_submit = {
        let messages = messages.clone();
        Callback::from(move |content: String| {
            let message = Message {
                sender: "Me".to_string(),
                content,
                timestamp: chrono::Local::now().to_rfc2822(),
            };

            messages.set(
                messages
                    .iter()
                    .cloned()
                    .chain(std::iter::once(message))
                    .collect::<Vec<_>>(),
            );
        })
    };

    html! {
        <div>
            <div class="chat">
                {for messages.iter().map(|message| {
                    html! {
                        <MessageComp message={message.clone()} />
                    }
                })}
            </div>
            <Input {on_submit} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
