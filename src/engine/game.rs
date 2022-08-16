pub trait Game {
    fn init(&mut self);
    fn update(&mut self);
    fn render(&mut self);
    fn finalize(self);
}
