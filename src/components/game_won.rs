use crate::{constant::Player, stores::game_state::GameState};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(GameWon)]
pub fn header() -> Html {
    let (state, _) = use_store::<GameState>();
    let (_, dispatch) = use_store::<GameState>();
    let onclick: Callback<MouseEvent> = dispatch.reduce_mut_callback(move |state_store| {
        state_store.restart_game();
    });
    html! {
        <div>
            {
                match state.winning_player {
                    Some(Player::PlayerTop) => {
                        html!{<h2>{"Le joueur du haut a gagné !"}</h2>}
                    }
                    Some(Player::PlayerBottom) => {
                        html!{<h2>{"Le joueur du bas a gagné !"}</h2>}
                    }
                    _ => html!{}
                }
            }
            <button onclick={onclick.clone()}>{"Relancer le jeu ?"}</button>
        </div>
    }
}
