use std::time::Duration;

use bronze::{
    cursor::Cursor,
    engine::Engine,
    game::Game,
    graphics::{Canvas, Sprite},
    icon::Icon,
    input::{InputManager, Key},
    resources::ResourcePool,
    sfml::graphics::Color,
    window::{FPSConfig, Window, WindowConfig},
};

use debugger::Debugger;
use resources::*;

mod debugger;
mod resources;

pub struct MyGame<'r> {
    bg: Sprite<'r>,
    debugger: Debugger<'r>,
    running: bool,
}

impl<'r> MyGame<'r> {
    fn new(resource_pool: &'r ResourcePool<Images, Audios, Fonts>) -> Self {
        MyGame {
            bg: Sprite::new(&resource_pool.get_image(Images::Background)),
            debugger: Debugger::new(true, resource_pool.get_font(Fonts::Debug), 10),
            running: true,
        }
    }
}

impl<'r> Game for MyGame<'r> {
    fn is_running(&self) -> bool {
        self.running
    }

    fn input(&mut self, input: &InputManager) {
        if input.key_down(Key::Escape) {
            self.running = false;
        }
        self.debugger.input(input);
    }

    fn update(&mut self, engine: &mut Engine, delta: Duration) {
        self.debugger.update(engine, delta);
    }

    fn draw<C: Canvas>(&self, target: &mut C) {
        self.bg.draw(target);
        self.debugger.draw(target);
    }
}

fn main() {
    let resource_pool = ResourcePool::new(load_image, load_audio, load_font);

    let win_config = WindowConfig {
        title: "My Game".to_string(),
        icon: Some(Icon::from_image(resource_pool.get_image(Images::Icon))),
        cursor: Cursor::from_image(resource_pool.get_image(Images::Cursor)),
        bg_color: Color::BLACK,
        show_cursor: true,
        fps_config: FPSConfig::VSync,
        mode: (960, 540).into(),
    };

    let mut engine = Engine::new(Window::new(win_config));

    let game = MyGame::new(&resource_pool);
    engine.run(game);
}
