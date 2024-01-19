use yew::prelude::*;
use yew::{Html,function_component};

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html!{
        <div>
        {"Hola"}
        </div>
    }

}