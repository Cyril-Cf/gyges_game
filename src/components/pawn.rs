use crate::constant::PawnType;
use crate::constant::Player;
use crate::stores::game_state::GameState;
use crate::stores::game_state::Pawn;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub pawn: Option<Pawn>,
    pub index: usize
}

#[function_component(PawnRender)]
pub fn pawn_render(props: &Props) -> Html {
    let Props { pawn, index } = props.clone();
    let (_, dispatch) = use_store::<GameState>();
    let onclick: Callback<MouseEvent> = dispatch.reduce_mut_callback(move |state_store| {
        state_store.select_or_move_pawn(index);
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
