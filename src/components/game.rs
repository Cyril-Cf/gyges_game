use crate::components::pawn::PawnRender;
use crate::stores::game_state::GameState;
use yew::prelude::*;
use yewdux::prelude::*;


#[function_component(Game)]
pub fn game() -> Html {
    let (_, dispatch) = use_store::<GameState>();
    let (state, _) = use_store::<GameState>();
    let onclick: Callback<MouseEvent> = dispatch.reduce_mut_callback(|counter| counter.count += 1);

    html! {
        <>
            <p>{state.count}</p>
            <section id="game">
                <div class="finish_line" id="player1_finish">
                    <div class="grid-item"></div>
                </div>
                <div class="grid-container">
                {
                    state.board.into_iter().map(|option| {
                        match option {
                            Some(pawn) => html!{<PawnRender pawn={pawn} onclick={onclick.clone()} />},
                            None => html!{<div class="grid-item"></div>}
                        }
                    }).collect::<Html>()
                }
                </div>
                <div class="finish_line" id="player2_finish">
                    <div class="grid-item"></div>
                </div>
            </section>
        </>
    }
}
