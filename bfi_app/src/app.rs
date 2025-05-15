use yew::{Html, function_component, html};

mod content;
mod footer;
mod header;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}
