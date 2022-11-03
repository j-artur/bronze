use std::time::Duration;

use strum_macros::EnumIter;

use bronze::{
    cursor::Cursor,
    debugger::Debugger,
    engine::Engine,
    game::Game,
    graphics::{Canvas, Sprite},
    icon::Icon,
    input::{InputManager, Key},
    resources::{Audio, Font, Image, ResourcePool},
    window::{Window, WindowConfig},
};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, EnumIter)]
enum Images {
    Background,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
enum Audios {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
enum Fonts {
    Debug,
}

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

    fn draw(&self, target: &mut dyn Canvas) {
        self.bg.draw(target);
        self.debugger.draw(target);
    }
}

fn main() {
    let win_config = WindowConfig {
        title: "My Game".to_string(),
        icon: Icon::from_image("assets/images/icon.png"),
        cursor: Cursor::from_image("assets/images/cursor.png"),
        mode: (960, 540).into(),
        ..WindowConfig::default()
    };

    let mut engine = Engine::new(Window::new(win_config));

    let resource_pool = ResourcePool::new(load_image, load_audio, load_font);

    let game = MyGame::new(&resource_pool);
    engine.run(game);
}

fn load_image(id: &Images) -> Option<Image> {
    use Images::*;
    match id {
        Background => Image::new("assets/images/bg.jpg"),
    }
}

fn load_audio(id: &Audios) -> Option<Audio> {
    // use Audios::*;
    match id {
        _ => None,
    }
}

fn load_font(id: &Fonts) -> Option<Font> {
    use Fonts::*;
    match id {
        Debug => Font::new("assets/fonts/JetBrainsMono[wght].ttf"),
    }
}
