use crate::components::Input;
use crate::model::{Message, MessageSender};
use std::sync::Arc;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct MessageInputProps {
    pub channel: String,
    pub sender: Arc<dyn MessageSender>,
    pub current_user: String,
}

impl PartialEq for MessageInputProps {
    fn eq(&self, other: &Self) -> bool {
        self.current_user == other.current_user && Arc::ptr_eq(&self.sender, &other.sender)
    }
}

#[function_component(MessageInputComp)]
pub fn message_input_comp(props: &MessageInputProps) -> Html {
    let on_submit = {
        let channel = props.channel.clone();
        let sender = props.sender.clone();
        let current_user = props.current_user.clone();

        Callback::from(move |message: String| {
            let message = Message {
                id: None,
                sender: current_user.clone(),
                content: message,
                timestamp: chrono::Utc::now(),
            };
            wasm_bindgen_futures::spawn_local({
                let channel = channel.clone();
                let sender = sender.clone();
                async move {
                    match sender.send_message(&channel, message).await {
                        Ok(_) => {
                            log::info!("Message sent successfully");
                        }
                        Err(e) => {
                            log::error!("Failed to send message: {:?}", e);
                        }
                    }
                }
            });
        })
    };

    html! {
        <Input {on_submit} />
    }
}
