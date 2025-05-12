use yew::{Html, Renderer, function_component, html};

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

fn main() {
    Renderer::<App>::new().render();
}
