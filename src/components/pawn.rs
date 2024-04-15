use crate::constant::PawnType;
use crate::constant::Player;
use crate::entities::board::Pawn;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub pawn: Option<Pawn>,
}

#[function_component(PawnRender)]
pub fn pawn_render(props: &Props) -> Html {
    let Props { pawn } = props;

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

            let mut class = classes!("grid-item");
            if pawn.is_highlighted {
                class.push("pawn_highlighted");
            }

            html! {
                <div {class} >
                    <div class={classes!(player, "pawn")}>
                        {text}
                    </div>
                </div>
            }
        }
    }
}
