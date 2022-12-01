use std::{rc::Rc, time::Duration};

use bronze::{
    graphics::{Color, Text},
    input::{InputManager, Key},
    resources::Font,
    scene::Entity,
    shape::ShapeRef,
    window::Canvas,
};

use crate::GameContext;

pub struct Debugger {
    text: Text,
    on: bool,
    total_time: f32,
    frames: u32,
}

impl Debugger {
    pub fn new(on: bool, font: &Rc<Font>, font_size: u32) -> Self {
        Debugger {
            text: Text::new(font, font_size, String::new()),
            on,
            total_time: 0.0,
            frames: 0,
        }
    }

    pub fn toggle(&mut self) {
        self.on = !self.on;
    }
}

impl Entity for Debugger {
    type Ctx = GameContext;

    #[inline]
    fn input(&mut self, input: &InputManager) {
        if input.key_pressed(Key::F) && input.key_down(Key::LControl) {
            self.toggle();
        }
    }

    #[inline]
    fn update(&mut self, _ctx: &mut GameContext, frame_time: Duration) {
        if self.on {
            self.total_time += frame_time.as_secs_f32();
            self.frames += 1;
            if self.total_time >= 0.250 {
                self.text.set_string(format!(
                    "FPS: {:.02}\nFrame Time: {:.02}ms",
                    self.frames as f32 / self.total_time,
                    frame_time.as_secs_f32() * 1000.0
                ));
                self.frames = 0;
                self.total_time = 0.0;
            }
        }
    }

    #[inline]
    fn draw(&self, _ctx: &GameContext, target: &mut Canvas) {
        if self.on {
            self.text.draw(target, (10.0, 10.0, Color::WHITE));
        }
    }

    #[inline]
    fn bbox(&self) -> ShapeRef {
        ShapeRef::None
    }
}
