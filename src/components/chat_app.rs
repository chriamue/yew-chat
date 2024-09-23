use crate::components::{ChatComp, MessageInputComp};
use crate::handler::MessageHandler;
use crate::model::{Chat, Message, MessageReceiver, MessageSender, ReceiveError, SendError};
use async_trait::async_trait;
use std::sync::Arc;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct ChatAppProps {
    pub user: String,
    pub channel: String,
    pub handler: Arc<dyn MessageHandler>,
}

impl PartialEq for ChatAppProps {
    fn eq(&self, other: &Self) -> bool {
        self.user == other.user && self.channel == other.channel
    }
}

struct ChatAppMessageHandler {
    handler: Arc<dyn MessageHandler>,
}

#[async_trait(?Send)]
impl MessageSender for ChatAppMessageHandler {
    async fn send_message(&self, channel: &str, message: Message) -> Result<(), SendError> {
        self.handler.send_message(channel, message).await
    }
}

#[async_trait(?Send)]
impl MessageReceiver for ChatAppMessageHandler {
    async fn receive_messages(&self, channel: &str) -> Result<Vec<Message>, ReceiveError> {
        self.handler.receive_messages(channel).await
    }
}

#[function_component(ChatApp)]
pub fn chat_app(props: &ChatAppProps) -> Html {
    let channel = props.channel.clone();
    let user = props.user.clone();
    let handler = props.handler.clone();

    let chat = use_state(|| Chat::new(channel.clone()));

    let handler = Arc::new(ChatAppMessageHandler {
        handler: handler.clone(),
    });

    let sender = handler.clone() as Arc<dyn MessageSender>;
    let receiver = handler.clone() as Arc<dyn MessageReceiver>;

    {
        let receiver = receiver.clone();
        let chat = chat.clone();
        let channel = channel.clone();

        use_effect_with(chat.clone(), move |_| {
            let chat = chat.clone();
            let channel = channel.clone();
            let receiver = receiver.clone();

            let interval = gloo::timers::callback::Interval::new(1000, move || {
                let chat = chat.clone();
                let channel = channel.clone();
                let receiver = receiver.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(messages) = receiver.receive_messages(&channel).await {
                        let mut updated_chat = (*chat).clone();
                        // add if not already in chat and sorted by timestamp
                        for message in messages {
                            if !updated_chat.messages.contains(&message) {
                                updated_chat.messages.push(message);
                            }
                        }
                        updated_chat.messages.sort_by_key(|m| m.timestamp);
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
            <MessageInputComp channel={channel.clone()} current_user={user} sender={sender.clone()} />
        </div>
    }
}
