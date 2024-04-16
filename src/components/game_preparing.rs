use crate::components::pawn::PawnRender;
use crate::constant::GameStatus;
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
    html! {
        <>
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
                                Some(pawn) => html!{<PawnRender {pawn} />},
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
            {
                match state.status {
                    GameStatus::Preparing(_) => {
                        html!{
                            <>
                                <button onclick={onclick_full_random.clone()}>{"Full random"}</button>
                                <button onclick={onclick_random_player_bottom.clone()}>{"Random joueur du bas"}</button>
                                <button onclick={onclick_random_player_top.clone()}>{"Random joueur du haut"}</button>
                            </>
                        }
                    },
                    _ => html!{}
                }

            }
        </>
    }
}
