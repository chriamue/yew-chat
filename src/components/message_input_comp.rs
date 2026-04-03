use crate::components::Input;
use crate::handler::MessageHandler;
use crate::model::Message;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct MessageInputProps<H: MessageHandler> {
    pub channel: String,
    pub handler: H,
    pub current_user: String,
}

#[function_component(MessageInputComp)]
pub fn message_input_comp<H: MessageHandler>(props: &MessageInputProps<H>) -> Html {
    let on_submit = {
        let channel = props.channel.clone();
        let handler = props.handler.clone();
        let current_user = props.current_user.clone();

        Callback::from(move |content: String| {
            let message = Message {
                id: None,
                sender: current_user.clone(),
                content,
                timestamp: chrono::Utc::now(),
            };
            wasm_bindgen_futures::spawn_local({
                let channel = channel.clone();
                let handler = handler.clone();
                async move {
                    if let Err(e) = handler.send_message(&channel, message).await {
                        log::error!("Failed to send message: {:?}", e);
                    }
                }
            });
        })
    };

    html! {
        <Input {on_submit} />
    }
}
