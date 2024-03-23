use crate::constant::PawnType;
use crate::constant::Player;
use crate::state::Pawn;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub pawn: Option<Pawn>,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(PawnRender)]
pub fn pawn_render(props: &Props) -> Html {
    let Props { pawn, onclick } = props.clone();

    match pawn {
        None => html! {
            <div class="grid-item"></div>
        },
        Some(pawn) => {
            let text = match pawn.paywn_type {
                PawnType::One => "1",
                PawnType::Two => "2",
                PawnType::Three => "3",
            };
            let player = match pawn.player {
                Player::PlayerOne => "player1",
                Player::PlayerTwo => "player2",
            };

            gloo_console::log!("testing log");

            html! {
                <div class="grid-item" onclick={onclick.clone()} >
                    <div class={classes!(player, "pawn")}>
                        {text}
                    </div>
                </div>
            }
        }
    }
}
