use crate::model::Message;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct MessageCompProps {
    pub message: Message,
}

#[function_component(MessageComp)]
pub fn message_comp(props: &MessageCompProps) -> Html {
    html! {
        <div class="message-comp">
            <div class="message-header">
                <span class="message-sender">{&props.message.sender}</span>
                <span class="message-timestamp">{&props.message.timestamp}</span>
            </div>
            <div class="message-content">{&props.message.content}</div>
        </div>
    }
}
