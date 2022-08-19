use std::thread::sleep;
use std::time::Duration;

use super::window::Window;

pub trait Game {
    fn init(&mut self, window: &Window);
    fn update(&mut self, window: &Window);
    fn render(&mut self, window: &Window);
    fn finalize(&mut self);

    fn on_pause(&mut self) {
        sleep(Duration::from_millis(10));
    }
}
