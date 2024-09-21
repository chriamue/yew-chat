use yew::prelude::*;
use yew_chat::prelude::Input;

#[function_component(App)]
fn app() -> Html {
    let on_submit = Callback::from(|message: String| {
        // Handle the submitted message here
        log::info!("Message submitted: {}", message);
    });

    html! {
        <div>
            // Other components...
            <Input {on_submit} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
