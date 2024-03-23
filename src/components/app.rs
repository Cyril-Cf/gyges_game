use yew::prelude::*;
use yew::{function_component, html};

use crate::components::aside::Aside;
use crate::components::footer::Footer;
use crate::components::game::Game;
use crate::components::header::Header;

use crate::state::{Action, State};

#[function_component]
pub fn App() -> Html {
    let state = use_reducer(State::reset);
    html! {
        <>
            <Header />
            <main>
                <Game board={state.board} />
                <Aside />
            </main>
            <Footer />
        </>
    }
}
