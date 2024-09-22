use crate::components::message_comp::MessageComp;
use crate::model::Chat;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct ChatCompProps {
    pub chat: Chat,
}

#[function_component(ChatComp)]
pub fn chat_comp(props: &ChatCompProps) -> Html {
    html! {
        <div class="chat-comp">
            {for props.chat.get_messages().iter().map(|message| {
                html! {
                    <MessageComp message={message.clone()} />
                }
            })}
        </div>
    }
}
