use crate::components::pawn::PawnRender;
use crate::constant::Player;
use crate::stores::game_state::GameState;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(GamePlaying)]
pub fn game() -> Html {
    let (state, _) = use_store::<GameState>();
    let (_, dispatch) = use_store::<GameState>();
    html! {
        <section id="game">
            <div class="finish_line" id="player1_finish">
                {
                    if state.board
                    .lines
                    .into_iter()
                    .filter(|l| l.is_hidden)
                    .next()
                    .unwrap()
                    .squares
                    .into_iter()
                    .any(|s| s.is_can_move_to) {
                        let onclick: Callback<MouseEvent> = dispatch.reduce_mut_callback(move |state_store| {
                            state_store.select_or_move_pawn(None, Some(Player::PlayerBottom));
                        });
                        html!{<div class="grid-item possible-move" onclick={onclick.clone()}></div>}
                    } else {
                        html!{<div class="grid-item"></div>}
                    }
                }
            </div>
            <div class="grid-container">
            {
                state.board.lines.into_iter().filter(|l| !l.is_hidden).map(|line| {
                    line.squares.into_iter().map(|square| {
                        let onclick: Callback<MouseEvent> = dispatch.reduce_mut_callback(move |state_store| {
                            state_store.select_or_move_pawn(Some(square), None);
                        });
                        let onmouseover: Callback<MouseEvent> = dispatch.reduce_mut_callback(move |state_store| {
                            state_store.trace_correct_path(&square);
                        });
                        let onmouseleave: Callback<MouseEvent> = dispatch.reduce_mut_callback(move |state_store| {
                            state_store.remove_all_correct_paths();
                        });
                        match square.pawn {
                            Some(pawn) => {
                                if square.is_correct_path {
                                    let is_correct_path = true;
                                    html!{<div onclick={onclick.clone()}><PawnRender {pawn} {is_correct_path} /></div>}
                                } else {
                                    let is_correct_path = false;
                                    html!{<div onclick={onclick.clone()}><PawnRender {pawn} {is_correct_path} /></div>}
                                }
                            },
                            None => {
                                let mut class = classes!("grid-item");
                                if square.is_correct_path {
                                    class.push("correct-step");
                                } else if square.is_can_move_to {
                                    class.push("possible-move");
                                }
                                if square.is_can_move_to {
                                    html!{<div {class} onclick={onclick.clone()} onmouseover={onmouseover.clone()} onmouseleave={onmouseleave.clone()}></div>}
                                } else if square.is_correct_path {
                                    html!{<div {class}></div>}
                                } else {
                                    html!{<div {class}></div>}
                                }
                            }
                        }
                    }).collect::<Html>()
                }).collect::<Html>()
            }
            </div>
            <div class="finish_line" id="player2_finish">
            {
                if state.board
                .lines
                .into_iter()
                .filter(|l| l.is_hidden)
                .last()
                .unwrap()
                .squares
                .into_iter()
                .any(|s| s.is_can_move_to) {
                    let onclick: Callback<MouseEvent> = dispatch.reduce_mut_callback(move |state_store| {
                        state_store.select_or_move_pawn(None, Some(Player::PlayerTop));
                    });
                    html!{<div class="grid-item possible-move" onclick={onclick.clone()}></div>}
                } else {
                    html!{<div class="grid-item"></div>}
                }
            }
            </div>
        </section>
    }
}
