use std::time::Duration;

use sfml::SfBox;

use crate::{
    engine::Engine,
    game::Game,
    graphics::{Canvas, Color, Font, Text, Transformable},
    input::{InputManager, Key},
};

pub struct Debugger<'a> {
    on: bool,
    total_time: Duration,
    frames: u32,
    text: Text<'a>,
}

impl<'a> Debugger<'a> {
    pub fn new(on: bool, font: SfBox<Font>, size: u32) -> Self {
        let font: &'static SfBox<Font> = Box::leak(Box::new(font));

        let mut text = Text::new("", &font, size);
        text.set_position((10.0, 10.0));
        text.set_fill_color(Color::WHITE);

        Debugger {
            on,
            total_time: Duration::from_secs(0),
            frames: 0,
            text,
        }
    }

    pub fn toggle(&mut self) {
        self.on = !self.on;
    }
}

impl<'a> Game for Debugger<'a> {
    fn is_running(&self) -> bool {
        true
    }

    fn input(&mut self, input: &InputManager) {
        if input.key_press(Key::F) && input.key_down(Key::LControl) {
            self.toggle();
        }
    }

    fn update(&mut self, _: &mut Engine, frame_time: Duration) {
        self.total_time += frame_time;
        self.frames += 1;
        if self.total_time >= Duration::from_millis(200) {
            self.text.set_string(&format!(
                "FPS: {:.02}\nFrame Time: {:.02}ms",
                self.frames as f32 / self.total_time.as_secs_f32(),
                frame_time.as_secs_f64() * 1000.0
            ));
            self.frames = 0;
            self.total_time = Duration::from_secs(0);
        }
    }

    fn draw(&self, target: &mut dyn Canvas) {
        if self.on {
            target.draw(&self.text);
        }
    }
}
