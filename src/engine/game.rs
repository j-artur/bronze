use std::thread::sleep;
use std::time::Duration;

use super::context::Context;
use super::window::Window;

pub trait Game {
    fn init(&mut self, window: &mut Window);
    fn update(&mut self, ctx: Context);
    fn render(&mut self, ctx: Context);
    fn finalize(&mut self);

    fn on_pause(&mut self) {
        sleep(Duration::from_millis(10));
    }
}
