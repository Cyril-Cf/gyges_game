use yew::{function_component, html, Html};
use yewdux::prelude::*;

use crate::components::game_info::GameInfo;
use crate::components::rules::Rules;
use crate::constant::GameStatus;
use crate::stores::game_state::GameState;

#[function_component(Aside)]
pub fn aside() -> Html {
    let (state, _) = use_store::<GameState>();
    html! {
        <aside>
            {
                match state.status {
                    GameStatus::Playing => html!{<GameInfo />},
                    _ => html!{}
                }
            }

            <Rules />
        </aside>
    }
}
