use yew::{function_component, html, Html};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header >
            <img src="public/ferris.png"/>
            <h1>{"Apprenez et jouez à Gygès avec Rust !"}</h1>
        </header>
    }
}
