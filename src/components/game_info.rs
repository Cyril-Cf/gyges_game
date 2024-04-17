use crate::{constant::Player, stores::game_state::GameState};
use yew::{function_component, html, Html};
use yewdux::prelude::*;

#[function_component(GameInfo)]
pub fn game_info() -> Html {
    let (state, _) = use_store::<GameState>();
    html! {
        <section id="game_infos">
            <section id="player-turn">
                <h2>{"Tour du joueur : "}</h2>
                {
                    if state.player_turn == Player::PlayerTop {
                        html! {
                            <strong>{"Joueur du haut"}</strong>
                        }
                    } else {
                        html! {
                            <strong>{"Joueur du bas"}</strong>
                        }
                    }
                }
            </section>
            <section id="ux-infos">
                <h2>{"Déroulé d'un tour"}</h2>
                <ol>
                    <li>{"Sélectionnez un pion que vous pouvez déplacer"}</li>
                    <li>{"Les mouvements possibles apparaissent en vert"}</li>
                    <li>{"Choisissez l'emplacement d'arrivée"}</li>
                </ol>
            </section>
        </section>
    }
}
