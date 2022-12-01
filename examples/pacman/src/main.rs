use bronze::{
    engine::Engine,
    resources::ResourcePool,
    window::{Window, WindowConfig},
};

mod entities;
mod game;
mod resources;

use crate::{game::Game, resources::*};

pub const WINDOW_WIDTH: u32 = 960;
pub const WINDOW_HEIGHT: u32 = 720;

pub type Pool = ResourcePool<Images, Audios, Fonts>;

fn main() {
    let resource_pool = ResourcePool::new(load_image, load_audio, load_font);

    let win_config = WindowConfig {
        title: "Pacman".to_string(),
        mode: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
        ..Default::default()
    };

    let mut engine = Engine::new(Window::new(win_config));

    engine.run(Game::new(resource_pool));
}
