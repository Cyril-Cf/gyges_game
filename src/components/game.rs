use crate::components::pawn::PawnRender;
use crate::stores::game_state::GameState;
use yew::prelude::*;
use yewdux::prelude::*;


#[function_component(Game)]
pub fn game() -> Html {
    let (state, _) = use_store::<GameState>();
    html! {
        <section id="game">
            <div class="finish_line" id="player1_finish">
                <div class="grid-item"></div>
            </div>
            <div class="grid-container">
            {
                state.board.into_iter().enumerate().map(|(index, option)| {
                    match option {
                        Some(pawn) => html!{<PawnRender {pawn} {index} />},
                        None => html!{<div class="grid-item"></div>}
                    }
                }).collect::<Html>()
            }
            </div>
            <div class="finish_line" id="player2_finish">
                <div class="grid-item"></div>
            </div>
        </section>
    }
}
