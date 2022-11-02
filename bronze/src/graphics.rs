pub use sfml::graphics::*;

pub trait Canvas {
    fn draw<D: Drawable>(&mut self, drawable: &D);
}
