use yew::prelude::*;
use yew::{Html,function_component};

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let handle_input = Callback::from(|_| {});

    html!{
        <main>
            <div>
            {"Ingresa una palabra"}
            </div>
            <div>
            <input type="text" oninput={handle_input}/> 
            </div>
            <div>
            <button>{"busca!"}</button>
            </div>
        </main>
    }

}