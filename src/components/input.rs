use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub on_submit: Callback<String>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let input_ref = use_node_ref();

    let onsubmit = {
        let input_ref = input_ref.clone();
        let on_submit = props.on_submit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                let value = input.value();
                if !value.is_empty() {
                    on_submit.emit(value);
                    input.set_value("");
                }
            }
        })
    };

    html! {
        <form {onsubmit} class="input-form">
            <input
                type="text"
                ref={input_ref}
                placeholder="Type a message..."
                class="input-field"
            />
            <button type="submit" class="send-button">{"Send"}</button>
        </form>
    }
}
