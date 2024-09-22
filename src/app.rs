use async_trait::async_trait;
use std::rc::Rc;
use yew::prelude::*;
use yew_chat::prelude::{
    Chat, ChatComp, Message, MessageInputComp, MessageReceiver, MessageSender, ReceiveError,
    SendError,
};

struct SimpleMessageHandler {
    message_queue: UseStateHandle<Vec<Message>>,
}

#[async_trait(?Send)]
impl MessageSender for SimpleMessageHandler {
    async fn send_message(&self, message: Message) -> Result<(), SendError> {
        let mut queue = (*self.message_queue).clone();
        queue.push(message);
        self.message_queue.set(queue);
        Ok(())
    }
}

impl MessageReceiver for SimpleMessageHandler {
    fn receive_messages(&self, _channel: &str) -> Result<Vec<Message>, ReceiveError> {
        let queue = (*self.message_queue).clone();
        if !queue.is_empty() {
            self.message_queue.set(Vec::new());
            Ok(queue)
        } else {
            Err(ReceiveError::UnknownError)
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let user = "Me".to_string();
    let chat = use_state(|| Chat::new("channel1".to_string()));
    let message_queue = use_state(|| Vec::new());

    let handler = Rc::new(SimpleMessageHandler {
        message_queue: message_queue.clone(),
    });

    let sender = handler.clone() as Rc<dyn MessageSender>;
    let receiver = handler.clone() as Rc<dyn MessageReceiver>;

    {
        let receiver = receiver.clone();
        let chat = chat.clone();

        use_effect_with(vec![(*message_queue).clone()], move |_| {
            let chat = chat.clone();
            let receiver = receiver.clone();

            let interval = gloo::timers::callback::Interval::new(1000, move || {
                let chat = chat.clone();
                if let Ok(messages) = receiver.receive_messages("channel1") {
                    let mut updated_chat = (*chat).clone();
                    for message in messages {
                        updated_chat.add_message(message);
                    }
                    chat.set(updated_chat);
                }
            });

            || drop(interval)
        });
    }

    html! {
        <div>
            <ChatComp chat={(*chat).clone()} />
            <MessageInputComp current_user={user} sender={sender.clone()} />
        </div>
    }
}
