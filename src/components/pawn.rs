use crate::constant::PawnType;
use crate::constant::Player;
use crate::entities::board::Pawn;
use crate::entities::board::Square;
use crate::stores::game_state::GameState;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub square: Square,
    pub pawn: Option<Pawn>,
}


#[function_component(PawnRender)]
pub fn pawn_render(props: &Props) -> Html {
    let Props { square, pawn } = props;
    let (_, dispatch) = use_store::<GameState>();
    let square = square.clone();
    let onclick: Callback<MouseEvent> = dispatch.reduce_mut_callback(move |state_store| {
        state_store.select_or_move_pawn(square);
    });

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
                <div {class} onclick={onclick.clone()} >
                    <div class={classes!(player, "pawn")}>
                        {text}
                    </div>
                </div>
            }
        }
    }
}
