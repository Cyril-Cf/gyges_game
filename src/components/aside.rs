use yew::{function_component, html, Html};

use crate::components::game_info::GameInfo;
use crate::components::rules::Rules;


#[function_component(Aside)]
pub fn aside() -> Html {
    html! {
        <aside>
            <GameInfo />
            <Rules />
        </aside>
    }
}
