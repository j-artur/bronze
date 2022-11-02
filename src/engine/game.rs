use std::time::Duration;

use super::{engine::Engine, input::InputManager, window::Canvas};

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
