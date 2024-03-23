mod components;
mod constant;
mod helper;
mod state;
mod stores;

use crate::components::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
