use yew::{Html, function_component, html};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <h1>{ "Brainfuck Interpreter" }</h1>
    }
}
