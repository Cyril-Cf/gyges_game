use yew::{function_component, html, Html, Properties, classes};
use crate::state::Pawn;
use crate::constant::PawnType;
use crate::constant::Player;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub pawn: Option<Pawn>,
}


#[function_component(PawnRender)]
pub fn pawn_render(props: &Props) -> Html {
    let Props {
        pawn
    } = props.clone();
 
    match pawn {
        None =>     html! {
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
                Player::PlayerTwo => "player2"
            };
            html! {
                <div class="grid-item">
                    <div class={classes!(player, "pawn")}>
                        {text}
                    </div>
                </div>
            }
        } 
            
    }

}