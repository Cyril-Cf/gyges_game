use yew::{function_component, html, Html};
use yewdux::prelude::*;

use crate::components::aside_game_preparing::AsideGamePreparing;
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
                    GameStatus::Preparing(_) => html!{
                        <>
                            <AsideGamePreparing />
                            <Rules />
                        </>
                    },
                    GameStatus::Playing => html!{
                        <>
                            <GameInfo />
                            <Rules />
                        </>
                    },
                    GameStatus::Finished => html!{}
                }
            }
        </aside>
    }
}
