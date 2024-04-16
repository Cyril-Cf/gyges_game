use crate::components::pawn::PawnRender;
use crate::constant::{GameStatus, Player, PreparationStep};
use crate::stores::game_state::GameState;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(GamePreparing)]
pub fn header() -> Html {
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
        <div class="container">
            {
                match state.status {
                    GameStatus::Preparing(_) => {
                        html!{
                            <>
                                <button onclick={onclick_full_random.clone()}>{"Full random"}</button>
                                <button onclick={onclick_random_player_bottom.clone()}>{"Random joueur du bas"}</button>
                                <button onclick={onclick_random_player_top.clone()}>{"Random joueur du haut"}</button>
                                <button onclick={onclick_chose_player_top.clone()}>{"Joueur du haut commence"}</button>
                                <button onclick={onclick_chose_player_bottom.clone()}>{"Joueur du bas commence"}</button>
                                <button onclick={onclick_start_game.clone()}>{"Lancer la partie"}</button>
                                {
                                    match state.player_turn {
                                        Player::PlayerTop => html!{"Le joueur du haut commence"},
                                        Player::PlayerBottom => html!{"Le joueur du bas commence"}
                                    }
                                }
                            </>
                        }
                    },
                    _ => html!{}
                }

            }
            <section id="game">
                <div class="finish_line" id="player1_finish">
                    {
                        html!{<div class="grid-item"></div>}
                    }
                </div>
                <div class="grid-container">
                {
                    state.board.lines.into_iter().filter(|l| !l.is_hidden).map(|line| {
                        line.squares.into_iter().map(|square| {
                            let onclick: Callback<MouseEvent> = dispatch.reduce_mut_callback(move |state_store| {
                                state_store.select_or_move_pawn(Some(square), None);
                            });
                            match square.pawn {
                                Some(pawn) => {
                                    let is_correct_path = false;
                                    html!{<PawnRender {pawn} {is_correct_path} />}
                                },
                                None => {
                                    html!{<div class="grid-item" onclick={onclick.clone()}></div>}
                                }
                            }
                        }).collect::<Html>()
                    }).collect::<Html>()
                }
                </div>
                <div class="finish_line" id="player2_finish">
                    {
                        html!{<div class="grid-item"></div>}
                    }
                </div>
            </section>
        </div>
    }
}
