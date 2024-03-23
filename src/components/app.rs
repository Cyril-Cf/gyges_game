use yew::prelude::*;
use yew::{function_component, html};

use crate::components::aside::Aside;
use crate::components::footer::Footer;
use crate::components::game::Game;
use crate::components::header::Header;

#[function_component]
pub fn App() -> Html {
    html! {
        <>
            <Header />
            <main>
                <Game />
                <Aside />
            </main>
            <Footer />
        </>
    }
}
