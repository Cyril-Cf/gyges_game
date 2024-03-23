use yew::{function_component, html, Html};

#[function_component(GameInfo)]
pub fn game_info() -> Html {
    html! {
        <section id="game_infos">
            {"Game infos"}
        </section>
    }
}
