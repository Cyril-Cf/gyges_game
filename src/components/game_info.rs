use crate::{constant::Player, stores::game_state::GameState};
use yew::{function_component, html, Html};
use yewdux::prelude::*;

#[function_component(GameInfo)]
pub fn game_info() -> Html {
    let (state, _) = use_store::<GameState>();
    html! {
        <section id="game_infos">
            {"Tour du joueur : "}
            {
                if state.player_turn == Player::PlayerTop {
                    html! {
                        <div>{"Joueur du haut"}</div>
                    }
                } else {
                    html! {
                        <div>{"Joueur du bas"}</div>
                    }
                }
            }
        </section>
    }
}
