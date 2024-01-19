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
            <div>
            <iframe width="560" height="315" src="https://www.youtube.com/embed/pvc84wrTJBI?si=42oHI4YIN9k1fnj9"></iframe>
            </div>
        </main>
    }

}