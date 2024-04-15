use crate::components::pawn::PawnRender;
use crate::stores::game_state::GameState;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Game)]
pub fn game() -> Html {
    let (state, _) = use_store::<GameState>();
    let (_, dispatch) = use_store::<GameState>();
    html! {
        <section id="game">
            <div class="finish_line" id="player1_finish">
                <div class="grid-item"></div>
            </div>
            <div class="grid-container">
            {
                state.board.lines.into_iter().filter(|l| !l.is_hidden).map(|line| {
                    line.squares.into_iter().map(|square| {
                        let onclick: Callback<MouseEvent> = dispatch.reduce_mut_callback(move |state_store| {
                            state_store.select_or_move_pawn(square);
                        });
                        match square.pawn {
                            Some(pawn) => html!{<div onclick={onclick.clone()}><PawnRender {pawn} /></div>},
                            None => {
                                if square.is_can_move_to {
                                    html!{<div class="grid-item possible-move" onclick={onclick.clone()}></div>}
                                } else {
                                    html!{<div class="grid-item"></div>}
                                }
                            }
                        }
                    }).collect::<Html>()
                }).collect::<Html>()
            }
            </div>
            <div class="finish_line" id="player2_finish">
                <div class="grid-item"></div>
            </div>
        </section>
    }
}
