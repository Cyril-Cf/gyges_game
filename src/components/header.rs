use yew::{function_component, html, Html};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header >
            <h1>{"Apprenez et jouez à Gygès !"}</h1>
        </header>
    }
}
