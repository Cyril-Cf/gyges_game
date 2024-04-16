use yew::prelude::*;
use yew::{function_component, html};
use yewdux::prelude::*;

use crate::components::aside::Aside;
use crate::components::footer::Footer;
use crate::components::game::Game;
use crate::components::game_won::GameWon;
use crate::components::header::Header;
use crate::constant::GameStatus;
use crate::stores::game_state::GameState;

#[function_component]
pub fn App() -> Html {
    let (state, _) = use_store::<GameState>();
    html! {
        <>
            <Header />
            <main>
                {
                    if state.status == GameStatus::Playing {
                        html!{<Game />}
                    } else {
                        html!{<GameWon />}
                    }
                }
                <Aside />
            </main>
            <Footer />
        </>
    }
}
