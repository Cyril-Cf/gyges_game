use crate::{
    constant::{GameStatus, Player, PreparationStep},
    stores::game_state::GameState,
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(AsideGamePreparing)]
pub fn footer() -> Html {
    let (state, _) = use_store::<GameState>();
    let (_, dispatch) = use_store::<GameState>();
    let onclick_full_random: Callback<MouseEvent> =
        dispatch.reduce_mut_callback(move |state_store| {
            state_store.place_pawns_full_random();
        });
    let onclick_random_player_top: Callback<MouseEvent> =
        dispatch.reduce_mut_callback(move |state_store| {
            state_store.place_pawns_random_player_top();
        });
    let onclick_random_player_bottom: Callback<MouseEvent> =
        dispatch.reduce_mut_callback(move |state_store| {
            state_store.place_pawns_random_player_bottom();
        });
    let onclick_start_game: Callback<MouseEvent> =
        dispatch.reduce_mut_callback(move |state_store| {
            if state_store.status == GameStatus::Preparing(PreparationStep::BothPlayersReady) {
                state_store.start_game();
            }
        });
    let onclick_chose_player_top: Callback<MouseEvent> =
        dispatch.reduce_mut_callback(move |state_store| {
            state_store.chose_player_top();
        });
    let onclick_chose_player_bottom: Callback<MouseEvent> =
        dispatch.reduce_mut_callback(move |state_store| {
            state_store.chose_player_bottom();
        });
    html! {
        <section id="game-preparation">
            {
                match state.status {
                    GameStatus::Preparing(_) => {
                        html!{
                            <>
                                <div class="container">
                                    <div>
                                        <h2>{"Placement des pions"}</h2>
                                        <button onclick={onclick_full_random.clone()}>{"Full random"}</button>
                                        <button onclick={onclick_random_player_top.clone()}>{"Random joueur du haut"}</button>
                                        <button onclick={onclick_random_player_bottom.clone()}>{"Random joueur du bas"}</button>
                                    </div>
                                    <div>
                                        <h2>{"Quel joueur va commencer ?"}</h2>
                                        <button onclick={onclick_chose_player_top.clone()}>{"Joueur du haut"}</button>
                                        <button onclick={onclick_chose_player_bottom.clone()}>{"Joueur du bas"}</button>
                                    </div>
                                    <div>
                                        <h2>{"Démarrage"}</h2>
                                        {
                                            match state.player_turn {
                                                Player::PlayerTop => html!{<p>{"Le joueur du haut va commencer !"}</p>},
                                                Player::PlayerBottom => html!{<p>{"Le joueur du bas va commencer !"}</p>}
                                            }
                                        }
                                        <button onclick={onclick_start_game.clone()}>{"Lancer la partie"}</button>
                                    </div>
                                </div>
                            </>
                        }
                    },
                    _ => html!{}
                }
            }
        </section>
    }
}
