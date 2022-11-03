use std::time::Duration;

use bronze::{
    cursor::Cursor,
    debugger::Debugger,
    engine::Engine,
    game::Game,
    graphics::{Canvas, Color, Font, RectangleShape, Shape, Transformable},
    icon::Icon,
    input::{InputManager, Key},
    window::{Window, WindowConfig},
};

pub struct MyGame<'a> {
    rectangle: RectangleShape<'a>,
    debugger: Debugger<'a>,
    running: bool,
}

impl<'a> MyGame<'a> {
    fn new() -> Self {
        let mut rectangle = RectangleShape::new();
        rectangle.set_size((100.0, 100.0));
        rectangle.set_fill_color(Color::RED);
        rectangle.set_position((100.0, 100.0));

        let font_path = "assets/fonts/JetBrainsMono[wght].ttf";

        let font =
            Font::from_file(font_path).expect(&format!("Failed to load font from {}", font_path));

        MyGame {
            rectangle,
            debugger: Debugger::new(true, font, 10),
            running: true,
        }
    }
}

impl<'a> Game for MyGame<'a> {
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
        if engine.input().key_down(Key::Left) {
            self.rectangle.move_((-200.0 * delta.as_secs_f32(), 0.0));
        }

        if engine.input().key_down(Key::Right) {
            self.rectangle.move_((200.0 * delta.as_secs_f32(), 0.0));
        }

        if engine.input().key_down(Key::Up) {
            self.rectangle.move_((0.0, -200.0 * delta.as_secs_f32()));
        }

        if engine.input().key_down(Key::Down) {
            self.rectangle.move_((0.0, 200.0 * delta.as_secs_f32()));
        }

        self.debugger.update(engine, delta);
    }

    fn draw(&self, target: &mut dyn Canvas) {
        target.draw(&self.rectangle);
        self.debugger.draw(target);
    }
}

fn main() {
    let win_config = WindowConfig {
        title: "My Game".to_string(),
        icon: Icon::from_image("assets/images/icon.png"),
        cursor: Cursor::from_image("assets/images/cursor.png"),
        mode: (800, 600).into(),
        ..WindowConfig::default()
    };

    let mut engine = Engine::new(Window::new(win_config));

    let game = MyGame::new();
    engine.run(game);
}
