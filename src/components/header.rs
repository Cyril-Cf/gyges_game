use yew::{function_component, html, Html};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header >
            {"Ceci est un header"}
        </header>
    }
}
