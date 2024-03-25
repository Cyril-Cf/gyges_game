mod components;
mod constant;
mod helper;
mod stores;
mod entities;

use crate::components::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
