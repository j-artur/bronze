use super::window::Window;

pub trait Game {
    fn init(&mut self, window: &Window);
    fn update(&mut self, window: &Window);
    fn render(&mut self, window: &Window);
    fn finalize(&mut self);
}
