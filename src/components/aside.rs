use yew::{classes, function_component, html, Html};
use yewdux::prelude::*;

use crate::components::aside_game_preparing::AsideGamePreparing;
use crate::components::game_info::GameInfo;
use crate::components::rules::Rules;
use crate::constant::GameStatus;
use crate::stores::game_state::GameState;

#[function_component(Aside)]
pub fn aside() -> Html {
    let (state, _) = use_store::<GameState>();
    let mut class = classes!("");
    if state.status == GameStatus::Finished {
        class.push("hidden");
    }
    html! {
        <aside {class}>
            {
                match state.status {
                    GameStatus::Preparing(_) => html!{
                        <div class="container">
                            <section id="top">
                                <AsideGamePreparing />
                            </section>
                            <section id="bottom">
                                <Rules />
                            </section>
                        </div>
                    },
                    GameStatus::Playing => html!{
                        <div class="container">
                            <section id="top">
                                <GameInfo />
                            </section>
                            <section id="bottom">
                                <Rules />
                            </section>
                        </div>
                    },
                    GameStatus::Finished => html!{}
                }
            }
        </aside>
    }
}
