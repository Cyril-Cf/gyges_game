use crate::constant::PawnType;
use crate::entities::board::Pawn;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub pawn: Option<Pawn>,
    pub is_correct_path: bool,
}

#[function_component(PawnRender)]
pub fn pawn_render(props: &Props) -> Html {
    let Props {
        pawn,
        is_correct_path,
    } = props;

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
            let mut class = classes!("grid-item");
            if *is_correct_path {
                class.push("correct-step");
            } else if pawn.is_highlighted {
                class.push("pawn_highlighted");
            }

            html! {
                <div {class} >
                    <div class={classes!("pawn")}>
                        {text}
                    </div>
                </div>
            }
        }
    }
}
