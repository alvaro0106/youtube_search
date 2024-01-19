use yew::prelude::*;
use yew::{Html,function_component};

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let on_search = Callback::<String>::from(|_| {});

    html!{
        <main>
            // <VideoControls on_search = {on_search} />
            <VideoSection name= "nombre video" id="pvc84wrTJBI?si=2SRySSjMjrJg3LfQ"/>
        </main>
    }
}

#[derive(Properties, PartialEq)]
struct VideoControlsProps {
    on_search: Callback<String>
}

#[function_component(VideoControls)]
fn controls(props: &VideoSectionProps) -> Html{
    let handle_input = Callback::from(|_| {});
    html!{
        <div>
            <div>
                {"Ingresa una palabra"}
            </div>
            <div>
                <input type="text" oninput={handle_input}/> 
            </div>
            <div>
                <button>{"busca!"}</button>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct VideoSectionProps{
    //Dos propiedades
    id: String,
    name: String,
}

#[function_component(VideoSection)]
fn video_section(props: &VideoSectionProps) -> Html {
    let yt_url: String =format!("https://www.youtube.com/embed/{}", props.id);
    html!{
        <div>
            <iframe width="560" height="315" src={yt_url}></iframe>
        </div>
    }
}