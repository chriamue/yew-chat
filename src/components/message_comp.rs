use crate::model::Message;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct MessageCompProps {
    pub message: Message,
}

#[function_component(MessageComp)]
pub fn message_comp(props: &MessageCompProps) -> Html {
    let formatted_timestamp = props.message.timestamp.format("%Y-%m-%d %H:%M").to_string();

    html! {
        <div class="message-comp">
            <div class="message-header">
                <span class="message-sender">{&props.message.sender}</span>
                <span class="message-timestamp">{formatted_timestamp}</span>
            </div>
            <div class="message-content">{&props.message.content}</div>
        </div>
    }
}
