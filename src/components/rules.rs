use yew::{function_component, html, Html};

#[function_component(Rules)]
pub fn rules() -> Html {
    html! {
        <section id="rules">
            {"Game rules"}
        </section>
    }
}
