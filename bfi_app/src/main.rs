use app::App;
use yew::Renderer;

mod app;

fn main() {
    Renderer::<App>::new().render();
}
