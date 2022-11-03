pub use sfml::graphics::*;

pub trait Canvas {
    fn draw(&mut self, drawable: &dyn Drawable);
}
