use std::time::Duration;

use crate::{engine::Engine, graphics::Canvas, input::InputManager};

pub trait Game {
    fn is_running(&self) -> bool;

    fn input(&mut self, input: &InputManager) {
        let _ = input;
    }

    fn pre_update(&mut self, engine: &Engine) {
        let _ = engine;
    }

    fn update(&mut self, engine: &mut Engine, frame_time: Duration) {
        let _ = (engine, frame_time);
    }

    fn post_update(&mut self, engine: &Engine) {
        let _ = engine;
    }

    fn draw<C: Canvas>(&self, target: &mut C) {
        let _ = target;
    }
}
