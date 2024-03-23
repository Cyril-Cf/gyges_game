use yew::{function_component, html, Html};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            {"Ceci est un footer"}
        </footer>
    }
}
