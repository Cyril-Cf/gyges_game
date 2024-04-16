mod components;
mod constant;
mod entities;
mod stores;

use crate::components::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
