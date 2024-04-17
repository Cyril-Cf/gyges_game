use yew::{function_component, html, Html};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <p>{"© 2024 Cyril COUFFE - Tous droits réservés."}</p>
            <p>{"Ce site a été développé comme base d'entraînement à la création de site web en WebAssembly avec "}<a href="https://yew.rs" target="_blank">{"Rust (Yew)"}</a>{"."}</p>
            <p>{"Si vous voulez en savoir plus sur moi, n'hésitez pas à visiter mon site principal "}<a href="https://formation-rust-web.fr" target="_blank">{"ici"}</a>{" !"}</p>
        </footer>
    }
}
