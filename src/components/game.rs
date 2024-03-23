use crate::components::pawn::PawnRender;
use crate::state::Pawn;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub board: [Option<Pawn>; 36],
}

#[function_component(Game)]
pub fn game(props: &Props) -> Html {
    let Props { board } = props.clone();

    let onclick = Callback::from(move |_: MouseEvent| {
        gloo_console::log!("Testing click!");
    });
    html! {
        <section id="game">
            <div class="finish_line" id="player1_finish">
                <div class="grid-item"></div>
            </div>
            <div class="grid-container">
            {
                board.into_iter().map(|option| {
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
    }
}
