use yew::{function_component, html, Html, Properties};
use gloo_console::log;

use crate::state::Pawn;
use crate::components::pawn::PawnRender;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub board: [Option<Pawn>; 36],
}

#[function_component(Game)]
pub fn game(props: &Props) -> Html {
    let Props {
        board
    } = props.clone();


    // let input_value_handle = use_state(String::default);
    // let input_value = (*input_value_handle).clone();

    // let onmouseenter = move |e: MouseEvent| {
    //     e.stop_propagation();
    // };
    log!("Testing..");


    html! {
        <section id="game">
            <div class="finish_line" id="player1_finish">
                <div class="grid-item"></div>
            </div>
            <div class="grid-container">
            {
                board.into_iter().map(|option| {
                    match option {
                        Some(pawn) => html!{<PawnRender pawn={pawn} />},
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
