use crate::components::pawn::PawnRender;
use crate::stores::game_state::GameState;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(GamePreparing)]
pub fn header() -> Html {
    let (state, _) = use_store::<GameState>();
    html! {
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
                        match square.pawn {
                            Some(pawn) => {
                                let is_correct_path = false;
                                html!{<PawnRender {pawn} {is_correct_path} />}
                            },
                            None => {
                                html!{<div class="grid-item"></div>}
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
    }
}
